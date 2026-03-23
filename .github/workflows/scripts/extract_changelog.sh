#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-}"
CHANGELOG="${2:-CHANGELOG.md}"

[[ -z "$VERSION" ]] && exit 1
[[ ! -f "$CHANGELOG" ]] && exit 1

# extract section for this version (between this ## and the next ##)
section="$(awk -v ver="$VERSION" '
    /^## / {
        if (found) exit
        if (index($0, ver)) { found=1; next }
    }
    found { print }
' "$CHANGELOG" \
	| sed '/<p align="right">/,/<\/p>/d' \
	| sed -e :a -e '/^[[:space:]]*$/{ $d; N; ba; }' \
    | sed '/^[[:space:]]*$/{ N; /^\n$/d; }')"

if [[ -z "$section" ]]; then
    echo "failed to extract release notes for '$VERSION' from $CHANGELOG" >&2
    exit 1
fi

printf '%s\n' "$section"
