#!/bin/bash

git submodule foreach git pull origin master
cargo protosgen
git add gcloud-sdk/genproto/*
git diff gcloud-sdk/genproto

#cd gcloud-sdk
#cargo release --package gcloud-sdk patch --no-dev-version --dry-run
#cd ..
