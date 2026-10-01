#!/usr/bin/env bash

# Shared by both installers; only top-level directories with SKILL.md are active.
parse_skill_install_options() {
  SKILLS_ONLY=false
  while [ "$#" -gt 0 ]; do
    case "$1" in
      --skills-only) SKILLS_ONLY=true; shift ;;
      --skills-dir)
        if [ "$#" -lt 2 ] || [ -z "$2" ]; then
          echo "Error: --skills-dir requires a destination directory" >&2
          return 1
        fi
        SKILLS_DST="$2"
        shift 2
        ;;
      *) echo "Error: unknown option: $1" >&2; return 1 ;;
    esac
  done
}

install_skill_links() {
  local skill_dir skill_name target current_link backup_dir=""
  if [ ! -d "$SKILLS_SRC" ]; then
    echo "Error: skills/ directory not found in $SCRIPT_DIR" >&2
    return 1
  fi
  mkdir -p "$SKILLS_DST"
  if [ "$(cd "$SKILLS_SRC" && pwd -P)" = "$(cd "$SKILLS_DST" && pwd -P)" ]; then
    echo "Error: skills destination must differ from the source directory" >&2
    return 1
  fi
  for skill_name in design ui-ux-pro-max auto-coding auto-sanity auto-layout auto-refactor; do
    target="$SKILLS_DST/$skill_name"
    if [ -e "$target" ] || [ -L "$target" ]; then
      if [ -z "$backup_dir" ]; then
        backup_dir="$(mktemp -d "$(dirname "$SKILLS_DST")/skills-backup-$(date +%Y%m%d-%H%M%S).XXXXXX")"
      fi
      mv "$target" "$backup_dir/$skill_name"
      echo "  retired: $skill_name -> $backup_dir/$skill_name"
    fi
  done
  # Previous Claude installs linked this grouping directory, exposing retired skills.
  target="$SKILLS_DST/future"
  if [ -L "$target" ] && [ "$(readlink "$target" | sed 's:/*$::')" = "$SKILLS_SRC/future" ]; then
    if [ -z "$backup_dir" ]; then
      backup_dir="$(mktemp -d "$(dirname "$SKILLS_DST")/skills-backup-$(date +%Y%m%d-%H%M%S).XXXXXX")"
    fi
    mv "$target" "$backup_dir/future"
    echo "  retired: future -> $backup_dir/future"
  fi
  for skill_dir in "$SKILLS_SRC"/*; do
    [ -d "$skill_dir" ] && [ -f "$skill_dir/SKILL.md" ] || continue
    skill_name="$(basename "$skill_dir")"
    target="$SKILLS_DST/$skill_name"
    if [ -L "$target" ]; then
      current_link="$(readlink "$target")"
      if [ "${current_link%/}" = "$skill_dir" ]; then
        echo "  skip: $skill_name (already linked)"
        continue
      fi
    fi
    # -L also catches dangling links, which -e alone misses.
    if [ -e "$target" ] || [ -L "$target" ]; then
      if [ -z "$backup_dir" ]; then
        backup_dir="$(mktemp -d "$(dirname "$SKILLS_DST")/skills-backup-$(date +%Y%m%d-%H%M%S).XXXXXX")"
      fi
      mv "$target" "$backup_dir/$skill_name"
      echo "  backup: $skill_name -> $backup_dir/$skill_name"
    fi
    ln -s "$skill_dir" "$target"
    echo "  linked: $skill_name"
  done
  echo "Done. Skills linked to $SKILLS_DST"
  if [ -n "$backup_dir" ]; then
    echo "Backups saved to $backup_dir"
  fi
}
