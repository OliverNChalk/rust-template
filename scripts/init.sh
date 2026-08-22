#!/bin/sh
#
# Instantiate this template into a new project directory.
#
# Copies the tracked files (via `git archive`, so ignored/untracked files and
# .git itself are excluded by construction) and renames the template to the
# given project name.
#
# Only lays down files; creating the repo and the first commit is left to the
# caller.
#
# Usage: ./scripts/init.sh <path> [name]
#
# `name` defaults to the basename of <path>.

set -eu

usage() {
    echo "Usage: $0 <path> [name]" >&2
    exit 1
}

[ $# -ge 1 ] || usage
[ $# -le 2 ] || usage

DEST=$1
NAME=${2:-$(basename "$DEST")}

# Cargo package names permit alphanumerics, `-` and `_`. Reject anything else
# up front rather than emitting a project that cannot build.
if ! echo "$NAME" | grep -qE '^[A-Za-z][A-Za-z0-9_-]*$'; then
    echo "error: '$NAME' is not a valid crate name (expected [A-Za-z][A-Za-z0-9_-]*)" >&2
    exit 1
fi

# The identifier form, used for module/target names in Rust source.
NAME_IDENT=$(echo "$NAME" | tr '-' '_')

REPO=$(git -C "$(dirname "$0")" rev-parse --show-toplevel)

# Refuse to write into a populated directory. A bare `.git` is tolerated so
# this can populate a repo the user has already created or cloned.
if [ -d "$DEST" ]; then
    for entry in "$DEST"/* "$DEST"/.*; do
        case "${entry##*/}" in
            '*' | '.*' | . | .. | .git) continue ;;
        esac
        [ -e "$entry" ] || continue
        echo "error: '$DEST' already exists and is not empty" >&2
        exit 1
    done
fi

mkdir -p "$DEST"
DEST_ABS=$(cd "$DEST" && pwd)

# `git archive` emits only tracked files at HEAD. Uncommitted work in the
# template is deliberately not carried over.
git -C "$REPO" archive --format=tar HEAD | tar -x -C "$DEST_ABS"

# This script bootstraps the template; it has no purpose in the generated
# project.
rm -f "$DEST_ABS/scripts/init.sh"

# Rewrite both the hyphenated and underscored spellings.
find "$DEST_ABS" -type f -exec sed -i \
    -e "s/rust-template/$NAME/g" \
    -e "s/rust_template/$NAME_IDENT/g" \
    {} +

# The README documents the template itself; replace it with a minimal stub.
cat >"$DEST_ABS/README.md" <<EOF
# $NAME
EOF

echo "Initialized '$NAME' at $DEST_ABS"
