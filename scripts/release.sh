#!/usr/bin/env bash
# Interactive release script for mip.rs
# Requires: gum (included in nix dev shell)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

# Check gum is available
if ! command -v gum &>/dev/null; then
    echo "error: gum is required. Enter the nix dev shell first: nix develop"
    exit 1
fi

# Read current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
if [ -z "$CURRENT_VERSION" ]; then
    echo "error: could not read version from Cargo.toml"
    exit 1
fi

IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"

echo ""
gum style --foreground 212 --bold "mip release"
echo "Current version: v${CURRENT_VERSION}"
echo ""

# Select bump type
BUMP=$(gum choose "hotfix (${MAJOR}.${MINOR}.$((PATCH + 1)))" "minor (${MAJOR}.$((MINOR + 1)).0)" "major ($((MAJOR + 1)).0.0)")

case "$BUMP" in
    hotfix*) NEW_VERSION="${MAJOR}.${MINOR}.$((PATCH + 1))" ;;
    minor*)  NEW_VERSION="${MAJOR}.$((MINOR + 1)).0" ;;
    major*)  NEW_VERSION="$((MAJOR + 1)).0.0" ;;
    *)       echo "error: unexpected selection"; exit 1 ;;
esac

RELEASE_DATE=$(date '+%-d %b %Y')
TAG="v${NEW_VERSION}"

echo ""
gum style --foreground 220 "Planned changes:"
echo "  Version: v${CURRENT_VERSION} → v${NEW_VERSION}"
echo "  Changelog: ## Unreleased → ## v${NEW_VERSION} - ${RELEASE_DATE}"
echo "  Tag: ${TAG}"
echo "  Bookmark: ${TAG}"
echo ""

if ! gum confirm "Proceed with release?"; then
    echo "Aborted."
    exit 0
fi

# 1. Bump version in Cargo.toml
sed -i "0,/^version = \"${CURRENT_VERSION}\"/s//version = \"${NEW_VERSION}\"/" Cargo.toml
echo "✓ Cargo.toml version → ${NEW_VERSION}"

# 2. Update Cargo.lock
cargo generate-lockfile 2>/dev/null || true
echo "✓ Cargo.lock updated"

# 3. Stamp changelog
sed -i "s/^## Unreleased$/## Unreleased\n\n## v${NEW_VERSION} - ${RELEASE_DATE}/" CHANGELOG.md
echo "✓ CHANGELOG.md stamped: v${NEW_VERSION} - ${RELEASE_DATE}"

# 4. Describe in jj
jj describe -m "release v${NEW_VERSION}"
echo "✓ jj describe: release v${NEW_VERSION}"

# 5. Create new working copy so the release commit is immutable
jj new
echo "✓ jj new (clean working copy)"

# 6. Set jj bookmark on the release commit
jj bookmark set "${TAG}" -r @-
echo "✓ jj bookmark: ${TAG}"

# 7. Export to git and create tag
jj git export
git tag "${TAG}" "$(jj log -r @- --no-graph -T 'commit_id')"
echo "✓ git tag: ${TAG}"

echo ""
gum style --foreground 82 --bold "Release v${NEW_VERSION} ready!"
echo ""
echo "To push:"
echo "  jj git push --bookmark ${TAG}"
echo "  git push origin ${TAG}"
echo ""

if gum confirm "Push now?"; then
    jj git push --bookmark "${TAG}"
    git push origin "${TAG}"
    echo "✓ Pushed!"
else
    echo "Skipped push. Run the commands above when ready."
fi
