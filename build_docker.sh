#!/bin/bash

export VERSION=$(grep -Eo '^version = ".*"' Cargo.toml | cut -d\" -f2)
export PACKAGE_NAME=$(grep -Eo '^name = ".*"' Cargo.toml | cut -d\" -f2)

if [ -z "$VERSION" ]; then
  echo "No version found, exiting..."
  exit 1
fi

echo "Building docker image for $PACKAGE_NAME with tags sas:$VERSION and sas:latest"

docker build -t sas:"$VERSION" -t sas:latest ./
