#!/bin/sh

# collect crate names and paths
list="$(cargo metadata --no-deps --format-version 1 | jq -r '.workspace_members[]' | grep "^twilight")"

echo "$list" | while read -r crate ; do
    path="$(echo "$crate" | awk '{ print $3 }' | sed 's/^.*path\+file:\/\/\(.*\)).*$/\1/' | xargs realpath --relative-to `pwd`)"
    tag="$(echo "$crate" | awk '{ print $1 "-" $2 }')"

    git-cliff --include-path "$path/**/*.rs" --unreleased --prepend "$path/CHANGELOG.md" "$tag"..HEAD
done
