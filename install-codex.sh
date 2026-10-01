#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SKILLS_SRC="$SCRIPT_DIR/skills"
SKILLS_DST="${CODEX_HOME:-$HOME/.codex}/skills"

source "$SCRIPT_DIR/tools/skills/install-common.sh"
parse_skill_install_options "$@"
install_skill_links
