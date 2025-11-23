#!/bin/bash

VERSION=$(cat version.txt)

docker build -t sas:"$VERSION" -t sas:latest ./