#!/bin/bash

if [[ -z "${GH_PUSH_TOKEN}" ]]; then
  echo "Env GH_PUSH_TOKEN must be specified"
  exit 1
fi

git submodule foreach git pull origin master
cargo protosgen
git add gcloud-sdk/genproto/*

git update-index --refresh
git diff-index --quiet HEAD gcloud-sdk/genproto
PROTOS_CHANGED=$?

CURRENT_DATE=$(date '+%Y-%m-%d')

if [[ PROTOS_CHANGED -eq 1 ]]; then
  echo "Found changes in Google APIs to release"
  cd gcloud-sdk || exit
  git commit -m "Google APIs updated at ${CURRENT_DATE}"
  cargo release --package gcloud-sdk patch --no-dev-version --token $GH_PUSH_TOKEN
  cd ..
else
  echo "No changes are found in protos to release"
fi