#!/bin/sh

metadata=$(cargo metadata --no-deps --format-version 1)
root=$(echo "$metadata" | jq -r '.workspace_root')

echo "$metadata" \
    | jq -r '.workspace_members[]' \
    | grep "^twilight" \
    | while read -r name version _ ;
do
    tag="$name-$version"

    git-cliff --include-path "$root/$name/**/*.rs" --unreleased --prepend "$root/$name/CHANGELOG.md" "$tag"..HEAD
done
