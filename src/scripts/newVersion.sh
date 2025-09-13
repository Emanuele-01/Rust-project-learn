#!/bin/bash

lastVersion="$1"
version="$2"

sed -i "s/$lastVersion/$tag/g" ./cargo.toml
sed -i "s/$lastVersion/$tag/g" ./VERSION

# Stage and commit changes first
git add .
git commit -m "create new version: $version"

# Create and push the tag
git tag -a $version -m "create new version: $version"
git push origin $version

