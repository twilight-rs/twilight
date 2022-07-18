#!/bin/sh

metadata=$(cargo metadata --no-deps --format-version 1)
root=$(echo "$metadata" | jq -r '.workspace_root' | xargs realpath --relative-to `pwd`)

if [[ "$root" != "." ]]; then
    echo "Must be run from repository root"
fi

echo "$metadata" \
    | jq -r '.workspace_members[]' \
    | grep "^twilight" \
    | while read -r name version _ ;
do
    tag="$name-$version"

    git-cliff --include-path "$name/**/*.rs" --unreleased --prepend "$name/CHANGELOG.md" "$tag"..HEAD
done
