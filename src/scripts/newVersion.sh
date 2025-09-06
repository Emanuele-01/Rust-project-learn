#!/bin/bash

version="1.0.0"

git tag $version -a

git push origin --tags

git add .

git commit -m "create new version: $version"

git push origin $version

