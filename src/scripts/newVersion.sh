#!/bin/bash

version="1.0.0"

git add .

git commit -m "create new version: $version"

git tag $version -a

git push origin --tags

git push origin $version

