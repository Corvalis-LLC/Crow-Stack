use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

use crate::config::{BUDGET_HEADROOM, CHARS_PER_TOKEN};
use crate::deps;
use crate::metrics::{self, HotspotThresholds};
use crate::output::{
    AnalysisResult, AnalysisScope, Dependency, DependencyHub, FileAnalysis, Hotspot, HotspotFile,
    PlanningContext, PlanningResult, PriorityFile, Summary, Symbol, Warning,
};
use crate::overview;
use crate::parse;
use crate::ranking;
use crate::resolve;
use crate::symbols;
use crate::walk::{self, WalkOptions};

/// Full analysis result plus pretty-printed rendering for `--format pretty`.
pub struct AnalyzeOutput {
    pub result: AnalysisResult,
    pub pretty: String,
}

const PRIMARY_ENTRY_POINT_LIMIT: usize = 12;
const DEPENDENCY_HUB_LIMIT: usize = 12;
const HOTSPOT_FILE_LIMIT: usize = 12;
const PRIORITY_FILE_LIMIT: usize = 15;
const DIFF_SIBLING_LIMIT_PER_DIRECTORY: usize = 4;

/// Run the full project analysis pipeline used by the `analyze` subcommand.
pub fn analyze_project(
    root: &Path,
    walk_options: &WalkOptions,
    budget_tokens: Option<usize>,
    diff_range: Option<&str>,
) -> Result<AnalyzeOutput> {
    let mut walk_result =
        walk::discover_files(root, walk_options).context("discovering source files")?;
    let scoped = build_analysis_scope(root, &walk_result.files, diff_range);
    let (source_files, scope) = match scoped {
        Ok(Some(scoped_selection)) => (scoped_selection.files, Some(scoped_selection.scope)),
        Ok(None) => (walk_result.files.clone(), None),
        Err(error) => {
            walk_result.warnings.push(Warning {
                path: ".".into(),
                message: format!("diff scope ignored: {error}"),
            });
            (walk_result.files.clone(), None)
        }
    };
    let parse_result = parse::parse_files(&source_files, root);
    let aliases = resolve::load_tsconfig_aliases(root);
    let graph = deps::build_dependency_graph(&parse_result.files, root, &aliases);

    let thresholds = HotspotThresholds::default();
    let mut files = Vec::new();
    let mut hotspots = Vec::new();

    for parsed in &parse_result.files {
        let file_symbols = symbols::extract_symbols(parsed);
        let file_metrics = metrics::analyze_file(parsed);
        hotspots.extend(metrics::detect_hotspots(
            &parsed.source_file.path,
            &file_metrics,
            &thresholds,
        ));

        files.push(FileAnalysis {
            path: parsed.source_file.path.clone(),
            language: parsed.source_file.language.as_str().to_string(),
            symbols: file_symbols.symbols,
            imports: file_symbols.imports,
            exports: file_symbols.exports,
            metrics: file_metrics,
        });
    }

    let ranked = ranking::score_files(&files, &graph, &hotspots);
    let mut ranked_files: Vec<FileAnalysis> = ranked
        .iter()
        .filter_map(|score| files.iter().find(|file| file.path == score.path).cloned())
        .collect();

    let project = overview::build_overview(root, &walk_result.files, &files, &graph.entry_points);
    let warnings = walk_result
        .warnings
        .into_iter()
        .chain(parse_result.warnings)
        .collect::<Vec<_>>();

    let summary = build_summary(&files);

    let mut result = AnalysisResult {
        version: env!("CARGO_PKG_VERSION").to_string(),
        project,
        scope,
        files: std::mem::take(&mut ranked_files),
        graph,
        hotspots,
        warnings,
        summary,
    };

    if let Some(budget) = budget_tokens {
        apply_budget(&mut result, budget);
    }

    let pretty = format_pretty(&result);

    Ok(AnalyzeOutput { result, pretty })
}

/// Build a compact planning-oriented payload from a full analysis result.
pub fn build_planning_result(result: &AnalysisResult) -> PlanningResult {
    let ranked = ranking::score_files(&result.files, &result.graph, &result.hotspots);
    let symbol_map = result
        .files
        .iter()
        .map(|file| (file.path.clone(), file.symbols.clone()))
        .collect();

    PlanningResult {
        version: result.version.clone(),
        project: result.project.clone(),
        scope: result.scope.clone(),
        symbols: symbol_map,
        dependencies: result.graph.adjacency.clone(),
        graph: result.graph.clone(),
        hotspots: result.hotspots.clone(),
        warnings: result.warnings.clone(),
        summary: result.summary.clone(),
        planning: build_planning_context(result, &ranked),
    }
}

fn build_summary(files: &[FileAnalysis]) -> Summary {
    let total_files = files.len();
    let total_symbols = files.iter().map(|file| file.symbols.len()).sum();
    let total_lines_of_code = files.iter().map(|file| file.metrics.code_lines).sum();
    let avg_complexity = if total_files == 0 {
        0.0
    } else {
        let total_complexity: u32 = files
            .iter()
            .map(|file| file.metrics.cyclomatic_complexity)
            .sum();
        ((total_complexity as f64 / total_files as f64) * 100.0).round() / 100.0
    };

    Summary {
        total_files,
        total_symbols,
        total_lines_of_code,
        avg_complexity,
    }
}

fn apply_budget(result: &mut AnalysisResult, budget_tokens: usize) {
    let target_chars =
        ((budget_tokens as f64) * (CHARS_PER_TOKEN as f64) * BUDGET_HEADROOM).floor() as usize;

    if target_chars == 0 {
        result.files.clear();
        return;
    }

    let fixed_cost = fixed_cost_chars(result);
    if fixed_cost >= target_chars {
        result.files.clear();
        return;
    }

    let full_files = std::mem::take(&mut result.files);
    let mut kept = Vec::new();
    let mut used_chars = fixed_cost;

    for file in full_files {
        let full_cost = serialized_len(&file);
        if used_chars + full_cost <= target_chars {
            used_chars += full_cost;
            kept.push(file);
            continue;
        }

        let summary_file = summarize_file(&file);
        let summary_cost = serialized_len(&summary_file);
        if used_chars + summary_cost <= target_chars {
            used_chars += summary_cost;
            kept.push(summary_file);
        } else {
            break;
        }
    }

    result.files = kept;
}

fn fixed_cost_chars(result: &AnalysisResult) -> usize {
    let fixed = serde_json::json!({
        "version": result.version,
        "project": result.project,
        "scope": result.scope,
        "files": [],
        "graph": result.graph,
        "hotspots": result.hotspots,
        "warnings": result.warnings,
        "summary": result.summary,
    });
    fixed.to_string().len()
}

fn serialized_len<T: serde::Serialize>(value: &T) -> usize {
    serde_json::to_string(value)
        .map(|json| json.len())
        .unwrap_or_default()
}

fn summarize_file(file: &FileAnalysis) -> FileAnalysis {
    FileAnalysis {
        path: file.path.clone(),
        language: file.language.clone(),
        symbols: file.symbols.iter().map(summarize_symbol).collect(),
        imports: Vec::new(),
        exports: file.exports.clone(),
        metrics: crate::output::FileMetrics {
            total_lines: file.metrics.total_lines,
            code_lines: file.metrics.code_lines,
            comment_lines: file.metrics.comment_lines,
            blank_lines: file.metrics.blank_lines,
            cyclomatic_complexity: file.metrics.cyclomatic_complexity,
            max_nesting_depth: file.metrics.max_nesting_depth,
            functions: Vec::new(),
        },
    }
}

fn summarize_symbol(symbol: &Symbol) -> Symbol {
    Symbol {
        name: symbol.name.clone(),
        kind: symbol.kind.clone(),
        line: symbol.line,
        end_line: symbol.line,
        exported: symbol.exported,
        signature: None,
    }
}

fn format_pretty(result: &AnalysisResult) -> String {
    let mut lines = Vec::new();
    let hotspot_counts = hotspot_counts(&result.hotspots);

    if let Some(scope) = &result.scope {
        let range = scope.diff_range.as_deref().unwrap_or("working tree");
        lines.push(format!(
            "Scope: {} ({} changed, {} total included)",
            range,
            scope.changed_files.len(),
            scope.included_files.len()
        ));
        lines.push(String::new());
    }

    for file in &result.files {
        let hotspot_count = hotspot_counts.get(file.path.as_str()).copied().unwrap_or(0);
        let mut header = format!(
            "{} (complexity: {}, {} symbols",
            file.path,
            file.metrics.cyclomatic_complexity,
            file.symbols.len()
        );
        if hotspot_count > 0 {
            header.push_str(&format!(", {} hotspots", hotspot_count));
        }
        header.push_str("):");
        lines.push(header);

        for symbol in &file.symbols {
            let descriptor = symbol
                .signature
                .as_deref()
                .map(|sig| format!("{} {}", symbol.name, sig))
                .unwrap_or_else(|| symbol.name.clone());
            lines.push(format!(
                "  {}: {:?} {}",
                symbol.line, symbol.kind, descriptor
            ));
        }

        if file.symbols.is_empty() {
            lines.push("  (no symbols)".to_string());
        }
    }

    if lines.is_empty() {
        lines.push("(no analyzed files)".to_string());
    }

    lines.join("\n")
}

#[derive(Debug, Clone)]
struct ScopedFileSelection {
    files: Vec<crate::output::SourceFile>,
    scope: AnalysisScope,
}

fn build_analysis_scope(
    root: &Path,
    all_files: &[crate::output::SourceFile],
    diff_range: Option<&str>,
) -> Result<Option<ScopedFileSelection>> {
    let Some(diff_range) = diff_range else {
        return Ok(None);
    };

    let changed_paths = git_diff_changed_paths(root, diff_range)?;
    if changed_paths.is_empty() {
        return Ok(None);
    }

    let files_by_path: BTreeMap<String, crate::output::SourceFile> = all_files
        .iter()
        .cloned()
        .map(|file| (file.path.clone(), file))
        .collect();

    let changed_files: Vec<crate::output::SourceFile> = changed_paths
        .iter()
        .filter_map(|path| files_by_path.get(path).cloned())
        .collect();

    if changed_files.is_empty() {
        return Ok(None);
    }

    let mut included_paths: BTreeSet<String> =
        changed_files.iter().map(|file| file.path.clone()).collect();
    let mut sibling_files = Vec::new();
    let mut truncated = false;

    for changed in &changed_files {
        let directory = changed
            .path
            .rsplit_once('/')
            .map(|(dir, _)| dir)
            .unwrap_or(".");

        let mut siblings: Vec<String> = all_files
            .iter()
            .filter(|file| file.path != changed.path)
            .filter(|file| {
                file.path
                    .rsplit_once('/')
                    .map(|(dir, _)| dir)
                    .unwrap_or(".")
                    == directory
            })
            .map(|file| file.path.clone())
            .collect();
        siblings.sort();

        if siblings.len() > DIFF_SIBLING_LIMIT_PER_DIRECTORY {
            truncated = true;
        }

        for sibling in siblings.into_iter().take(DIFF_SIBLING_LIMIT_PER_DIRECTORY) {
            if included_paths.insert(sibling.clone()) {
                sibling_files.push(sibling);
            }
        }
    }

    let initial_selection: Vec<crate::output::SourceFile> = included_paths
        .iter()
        .filter_map(|path| files_by_path.get(path).cloned())
        .collect();
    let parse_result = parse::parse_files(&initial_selection, root);
    let mut alias_cache = HashMap::new();
    let changed_set: BTreeSet<&str> = changed_files
        .iter()
        .map(|file| file.path.as_str())
        .collect();
    let mut imported_files = Vec::new();

    for parsed in &parse_result.files {
        if !changed_set.contains(parsed.source_file.path.as_str()) {
            continue;
        }

        let file_symbols = symbols::extract_symbols(parsed);
        let aliases = resolve::load_tsconfig_aliases_for_file(
            root,
            &parsed.source_file.path,
            &mut alias_cache,
        );

        for import in &file_symbols.imports {
            if let resolve::ResolvedImport::ProjectFile(target) =
                resolve::resolve_import(&import.source, &parsed.source_file.path, root, &aliases)
                && included_paths.insert(target.clone())
            {
                imported_files.push(target);
            }
        }

        for export in &file_symbols.exports {
            let Some(source) = export.source.as_deref() else {
                continue;
            };
            if let resolve::ResolvedImport::ProjectFile(target) =
                resolve::resolve_import(source, &parsed.source_file.path, root, &aliases)
                && included_paths.insert(target.clone())
            {
                imported_files.push(target);
            }
        }
    }

    let files: Vec<crate::output::SourceFile> = included_paths
        .iter()
        .filter_map(|path| files_by_path.get(path).cloned())
        .collect();

    let mut changed_paths_sorted: Vec<String> =
        changed_files.iter().map(|file| file.path.clone()).collect();
    changed_paths_sorted.sort();
    sibling_files.sort();
    imported_files.sort();
    let mut included_files: Vec<String> = included_paths.into_iter().collect();
    included_files.sort();

    Ok(Some(ScopedFileSelection {
        files,
        scope: AnalysisScope {
            kind: "diff".into(),
            diff_range: Some(diff_range.to_string()),
            changed_files: changed_paths_sorted,
            sibling_files,
            imported_files,
            included_files,
            truncated,
        },
    }))
}

fn git_diff_changed_paths(root: &Path, diff_range: &str) -> Result<Vec<String>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("diff")
        .arg("--name-only")
        .arg("--diff-filter=ACMR")
        .arg(diff_range)
        .arg("--")
        .output()
        .with_context(|| format!("running git diff for range '{diff_range}'"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "git diff failed for range '{}': {}",
            diff_range,
            stderr.trim()
        );
    }

    let mut paths: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.trim().replace('\\', "/"))
        .filter(|line| !line.is_empty())
        .collect();
    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn build_planning_context(
    result: &AnalysisResult,
    ranked: &[ranking::FileScore],
) -> PlanningContext {
    let hotspot_counts = hotspot_counts(&result.hotspots);
    let inbound = inbound_dependency_counts(&result.graph.adjacency);

    let primary_entry_points = ranked
        .iter()
        .filter(|score| score.is_entry_point)
        .take(PRIMARY_ENTRY_POINT_LIMIT)
        .map(|score| score.path.clone())
        .collect();

    let mut dependency_hubs: Vec<DependencyHub> = result
        .files
        .iter()
        .map(|file| DependencyHub {
            path: file.path.clone(),
            inbound_dependencies: *inbound.get(file.path.as_str()).unwrap_or(&0),
            export_count: file.exports.len(),
            hotspot_count: hotspot_counts.get(file.path.as_str()).copied().unwrap_or(0),
        })
        .filter(|hub| hub.inbound_dependencies > 0)
        .collect();
    dependency_hubs.sort_by(|a, b| {
        b.inbound_dependencies
            .cmp(&a.inbound_dependencies)
            .then_with(|| b.hotspot_count.cmp(&a.hotspot_count))
            .then_with(|| b.export_count.cmp(&a.export_count))
            .then_with(|| a.path.cmp(&b.path))
    });
    dependency_hubs.truncate(DEPENDENCY_HUB_LIMIT);

    let mut hotspot_files: Vec<HotspotFile> = result
        .files
        .iter()
        .filter_map(|file| {
            let hotspot_count = hotspot_counts.get(file.path.as_str()).copied().unwrap_or(0);
            if hotspot_count == 0 {
                return None;
            }

            Some(HotspotFile {
                path: file.path.clone(),
                hotspot_count,
                max_complexity: file.metrics.cyclomatic_complexity,
            })
        })
        .collect();
    hotspot_files.sort_by(|a, b| {
        b.hotspot_count
            .cmp(&a.hotspot_count)
            .then_with(|| b.max_complexity.cmp(&a.max_complexity))
            .then_with(|| a.path.cmp(&b.path))
    });
    hotspot_files.truncate(HOTSPOT_FILE_LIMIT);

    let file_map: BTreeMap<&str, &FileAnalysis> = result
        .files
        .iter()
        .map(|file| (file.path.as_str(), file))
        .collect();

    let priority_files = ranked
        .iter()
        .filter_map(|score| {
            let file = file_map.get(score.path.as_str())?;
            Some(PriorityFile {
                path: file.path.clone(),
                score: ((score.score * 100.0).round()) / 100.0,
                is_entry_point: score.is_entry_point,
                symbol_count: file.symbols.len(),
                complexity: file.metrics.cyclomatic_complexity,
            })
        })
        .take(PRIORITY_FILE_LIMIT)
        .collect();

    PlanningContext {
        primary_entry_points,
        dependency_hubs,
        hotspot_files,
        priority_files,
    }
}

fn hotspot_counts(hotspots: &[Hotspot]) -> BTreeMap<&str, usize> {
    let mut counts = BTreeMap::new();
    for hotspot in hotspots {
        *counts.entry(hotspot.path.as_str()).or_insert(0) += 1;
    }
    counts
}

fn inbound_dependency_counts(
    adjacency: &BTreeMap<String, Vec<Dependency>>,
) -> BTreeMap<&str, usize> {
    let mut inbound = BTreeMap::new();
    for deps in adjacency.values() {
        for dep in deps {
            if dep.resolved && !dep.external {
                *inbound.entry(dep.target.as_str()).or_insert(0) += 1;
            }
        }
    }
    inbound
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::{
        Dependency, DependencyGraph, Export, ExportKind, FileMetrics, FunctionMetrics, GraphStats,
        Import, ImportKind, ProjectOverview, SymbolKind, Warning,
    };

    fn make_file(
        path: &str,
        symbol_count: usize,
        export_count: usize,
        complexity: u32,
    ) -> FileAnalysis {
        FileAnalysis {
            path: path.to_string(),
            language: "typescript".to_string(),
            symbols: (0..symbol_count)
                .map(|idx| Symbol {
                    name: format!("sym_{idx}"),
                    kind: SymbolKind::Function,
                    line: idx + 1,
                    end_line: idx + 2,
                    exported: idx < export_count,
                    signature: Some("(x: number)".to_string()),
                })
                .collect(),
            imports: vec![Import {
                source: "./dep".to_string(),
                specifiers: vec!["dep".to_string()],
                kind: ImportKind::Named,
                line: 1,
            }],
            exports: (0..export_count)
                .map(|idx| Export {
                    name: format!("exp_{idx}"),
                    kind: ExportKind::Named,
                    line: idx + 1,
                    source: None,
                })
                .collect(),
            metrics: FileMetrics {
                total_lines: 50,
                code_lines: 40,
                comment_lines: 5,
                blank_lines: 5,
                cyclomatic_complexity: complexity,
                max_nesting_depth: 4,
                functions: vec![FunctionMetrics {
                    name: "sym_0".to_string(),
                    line: 1,
                    end_line: 10,
                    lines_of_code: 10,
                    cyclomatic_complexity: complexity,
                    max_nesting_depth: 4,
                    parameter_count: 2,
                }],
            },
        }
    }

    fn make_result(files: Vec<FileAnalysis>) -> AnalysisResult {
        AnalysisResult {
            version: "0.1.0".to_string(),
            project: ProjectOverview {
                name: "fixture".to_string(),
                root: ".".to_string(),
                languages: BTreeMap::new(),
                entry_points: vec!["src/main.ts".to_string()],
                config_files: vec!["package.json".to_string()],
                directory_tree: Vec::new(),
            },
            scope: None,
            files,
            graph: DependencyGraph {
                adjacency: BTreeMap::from([(
                    "src/main.ts".to_string(),
                    vec![Dependency {
                        target: "src/lib.ts".to_string(),
                        specifiers: vec!["lib".to_string()],
                        resolved: true,
                        external: false,
                    }],
                )]),
                entry_points: vec!["src/main.ts".to_string()],
                leaf_nodes: vec!["src/lib.ts".to_string()],
                cycles: Vec::new(),
                stats: GraphStats::default(),
            },
            hotspots: vec![Hotspot {
                path: "src/main.ts".to_string(),
                function: "sym_0".to_string(),
                metric: "cyclomatic_complexity".to_string(),
                value: 12,
                threshold: 10,
            }],
            warnings: vec![Warning {
                path: "src/generated.ts".to_string(),
                message: "skipped".to_string(),
            }],
            summary: Summary {
                total_files: 2,
                total_symbols: 8,
                total_lines_of_code: 80,
                avg_complexity: 6.0,
            },
        }
    }

    #[test]
    fn summary_counts_files_symbols_and_loc() {
        let files = vec![make_file("a.ts", 3, 2, 5), make_file("b.ts", 1, 0, 7)];
        let summary = build_summary(&files);

        assert_eq!(summary.total_files, 2);
        assert_eq!(summary.total_symbols, 4);
        assert_eq!(summary.total_lines_of_code, 80);
        assert_eq!(summary.avg_complexity, 6.0);
    }

    #[test]
    fn budget_keeps_fixed_sections_even_when_no_files_fit() {
        let mut result = make_result(vec![make_file("src/main.ts", 12, 6, 12)]);
        apply_budget(&mut result, 1);

        assert!(result.files.is_empty());
        assert_eq!(result.project.name, "fixture");
        assert_eq!(result.hotspots.len(), 1);
        assert_eq!(result.graph.entry_points, vec!["src/main.ts".to_string()]);
    }

    #[test]
    fn budget_prefers_higher_ranked_files() {
        let high = make_file("src/main.ts", 12, 6, 12);
        let low = make_file("src/leaf.ts", 1, 0, 1);
        let mut result = make_result(vec![high.clone(), low.clone()]);

        // Pre-sort like the real pipeline would.
        result.files = vec![high, low];
        apply_budget(&mut result, 4000);

        assert!(!result.files.is_empty());
        assert_eq!(result.files[0].path, "src/main.ts");
    }

    #[test]
    fn summarized_files_drop_heavy_details() {
        let file = make_file("src/main.ts", 4, 2, 12);
        let summarized = summarize_file(&file);

        assert!(summarized.imports.is_empty());
        assert!(summarized.metrics.functions.is_empty());
        assert!(
            summarized
                .symbols
                .iter()
                .all(|symbol| symbol.signature.is_none())
        );
    }

    #[test]
    fn pretty_format_includes_symbol_lines() {
        let result = make_result(vec![make_file("src/main.ts", 2, 1, 12)]);
        let pretty = format_pretty(&result);

        assert!(pretty.contains("src/main.ts (complexity: 12, 2 symbols"));
        assert!(pretty.contains("sym_0"));
    }

    #[test]
    fn analyze_end_to_end_builds_project_result() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("package.json"),
            r#"{"name":"fixture-project"}"#,
        )
        .unwrap();
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        std::fs::write(
            dir.path().join("src/main.ts"),
            "import { helper } from './shared/lib';\nexport function main() { return helper(); }\n",
        )
        .unwrap();
        std::fs::create_dir_all(dir.path().join("src/shared")).unwrap();
        std::fs::write(
            dir.path().join("src/shared/lib.ts"),
            "export function helper() { return 1; }\n",
        )
        .unwrap();

        let output = analyze_project(
            dir.path(),
            &WalkOptions {
                include: None,
                exclude: None,
            },
            None,
            None,
        )
        .unwrap();

        assert_eq!(output.result.project.name, "fixture-project");
        assert_eq!(output.result.summary.total_files, 2);
        assert_eq!(output.result.graph.stats.total_files, 2);
        assert_eq!(output.result.files[0].path, "src/main.ts");
        assert!(output.pretty.contains("src/main.ts"));
    }

    #[test]
    fn analyze_with_diff_scope_includes_changed_files_and_local_context() {
        use std::process::Command;

        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("package.json"),
            r#"{"name":"fixture-project"}"#,
        )
        .unwrap();
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        std::fs::write(
            dir.path().join("src/main.ts"),
            "import { helper } from './shared/lib';\nexport function main() { return helper(); }\n",
        )
        .unwrap();
        std::fs::create_dir_all(dir.path().join("src/shared")).unwrap();
        std::fs::write(
            dir.path().join("src/shared/lib.ts"),
            "export function helper() { return 1; }\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("src/helper.ts"),
            "export const sibling = true;\n",
        )
        .unwrap();

        let status = Command::new("git")
            .arg("-C")
            .arg(dir.path())
            .arg("init")
            .status()
            .unwrap();
        assert!(status.success());

        let status = Command::new("git")
            .arg("-C")
            .arg(dir.path())
            .args(["config", "user.email", "test@example.com"])
            .status()
            .unwrap();
        assert!(status.success());

        let status = Command::new("git")
            .arg("-C")
            .arg(dir.path())
            .args(["config", "user.name", "Test User"])
            .status()
            .unwrap();
        assert!(status.success());

        let status = Command::new("git")
            .arg("-C")
            .arg(dir.path())
            .args(["add", "."])
            .status()
            .unwrap();
        assert!(status.success());

        let status = Command::new("git")
            .arg("-C")
            .arg(dir.path())
            .args(["commit", "-m", "initial"])
            .status()
            .unwrap();
        assert!(status.success());

        std::fs::write(
            dir.path().join("src/main.ts"),
            "import { helper } from './shared/lib';\nexport function main() { return helper() + 1; }\n",
        )
        .unwrap();

        let output = analyze_project(
            dir.path(),
            &WalkOptions {
                include: None,
                exclude: None,
            },
            None,
            Some("HEAD"),
        )
        .unwrap();

        let scope = output.result.scope.expect("expected diff scope");
        assert_eq!(scope.kind, "diff");
        assert_eq!(scope.diff_range.as_deref(), Some("HEAD"));
        assert!(scope.changed_files.contains(&"src/main.ts".to_string()));
        assert!(
            scope
                .imported_files
                .contains(&"src/shared/lib.ts".to_string())
        );
        assert!(scope.sibling_files.contains(&"src/helper.ts".to_string()));
        assert!(scope.included_files.contains(&"src/main.ts".to_string()));
        assert!(
            scope
                .included_files
                .contains(&"src/shared/lib.ts".to_string())
        );
        assert!(scope.included_files.contains(&"src/helper.ts".to_string()));
    }

    #[test]
    fn planning_result_exposes_top_level_symbols_and_dependencies() {
        let result = make_result(vec![make_file("src/main.ts", 2, 1, 12)]);
        let planning = build_planning_result(&result);

        assert!(planning.symbols.contains_key("src/main.ts"));
        assert!(planning.dependencies.contains_key("src/main.ts"));
        assert_eq!(
            planning.planning.primary_entry_points,
            vec!["src/main.ts".to_string()]
        );
    }

    #[test]
    fn planning_context_curates_entry_points_and_hubs() {
        let result = AnalysisResult {
            version: "0.1.0".to_string(),
            project: ProjectOverview {
                name: "fixture".to_string(),
                root: ".".to_string(),
                languages: BTreeMap::new(),
                entry_points: vec!["app.ts".to_string(), "cli.ts".to_string()],
                config_files: Vec::new(),
                directory_tree: Vec::new(),
            },
            scope: None,
            files: vec![
                make_file("app.ts", 2, 2, 10),
                make_file("cli.ts", 1, 0, 1),
                make_file("lib.ts", 3, 3, 8),
            ],
            graph: DependencyGraph {
                adjacency: BTreeMap::from([
                    (
                        "app.ts".to_string(),
                        vec![Dependency {
                            target: "lib.ts".to_string(),
                            specifiers: vec!["run".to_string()],
                            resolved: true,
                            external: false,
                        }],
                    ),
                    (
                        "cli.ts".to_string(),
                        vec![Dependency {
                            target: "lib.ts".to_string(),
                            specifiers: vec!["run".to_string()],
                            resolved: true,
                            external: false,
                        }],
                    ),
                    ("lib.ts".to_string(), Vec::new()),
                ]),
                entry_points: vec!["app.ts".to_string(), "cli.ts".to_string()],
                leaf_nodes: vec!["lib.ts".to_string()],
                cycles: Vec::new(),
                stats: GraphStats::default(),
            },
            hotspots: vec![Hotspot {
                path: "app.ts".to_string(),
                function: "main".to_string(),
                metric: "cyclomatic_complexity".to_string(),
                value: 10,
                threshold: 8,
            }],
            warnings: Vec::new(),
            summary: Summary::default(),
        };

        let planning = build_planning_result(&result);

        assert_eq!(planning.planning.primary_entry_points[0], "app.ts");
        assert_eq!(planning.planning.dependency_hubs[0].path, "lib.ts");
        assert_eq!(planning.planning.hotspot_files[0].path, "app.ts");
    }
}
