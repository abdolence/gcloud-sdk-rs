#!/bin/bash

if [[ -z "${CARGO_REGISTRY_TOKEN}" ]]; then
  echo "Env CARGO_REGISTRY_TOKEN must be specified"
  exit 1
fi

git submodule update --init --recursive --recommend-shallow --depth 1
cargo protosgen
git add gcloud-sdk/genproto/*

git update-index --refresh
git diff-index --quiet HEAD gcloud-sdk/genproto
PROTOS_CHANGED=$?

CURRENT_DATE=$(date '+%Y-%m-%d')

if [[ PROTOS_CHANGED -eq 1 ]]; then
  echo "Found changes in Google APIs to release"
  cd gcloud-sdk || exit
  git config user.name "GitHub Actions"
  git config user.email "<>"
  git commit -m "Google APIs updated at ${CURRENT_DATE}"
  #cargo release --package gcloud-sdk patch --no-dev-version --token $CARGO_REGISTRY_TOKEN --dry-run
  git push origin master
  cd ..
else
  echo "No changes are found in protos to release"
fi
