#!/bin/bash

version="1.0.1"

# Stage and commit changes first
git add .
git commit -m "create new version: $version"

# Create and push the tag
git tag -a $version -m "create new version: $version"
git push origin $version

