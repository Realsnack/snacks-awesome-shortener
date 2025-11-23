#!/bin/bash

VERSION=$(grep -Eo '^version = ".*"' Cargo.toml | cut -d\" -f2)

if [ -z "$VERSION" ]; then
  echo "No version found, exiting..."
  exit 1
fi

echo "Running docker build with tags sas:$VERSION and sas:latest"

docker build -t sas:"$VERSION" -t sas:latest ./