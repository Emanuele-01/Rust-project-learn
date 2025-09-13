#!/bin/bash

lastVersion="$1"
version="$2"

sed -i '' "s|$lastVersion|$version|g" ./Cargo.toml
sed -i '' "s|v$lastVersion|v$version|g" ./VERSION

cargo update

# Stage and commit changes first
git add .
git commit -m "create new version: v$version"

# Create and push the tag
git tag -a v$version -m "create new version: v$version"
git push origin v$version

