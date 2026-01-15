#!/usr/bin/env bash
set -euo pipefail

# Calculate SHA256 from locally packaged crate
CRATE=$(ls target/package/*.crate)
SHA=$(sha256sum "$CRATE" | awk '{print $1}')

echo "SHA256: $SHA"
sed -i "s/^sha256sums=.*/sha256sums=('${SHA}')/" PKGBUILD

# Output for GitHub Actions
if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
  echo "sha256=$SHA" >> "$GITHUB_OUTPUT"
fi
