#!/bin/bash

#git submodule foreach git pull origin master
git submodule update --init --recursive --recommend-shallow --depth 1 --remote --merge
cargo protosgen
git add gcloud-sdk/genproto/*

git update-index --refresh
git diff-index --quiet HEAD gcloud-sdk/genproto
PROTOS_CHANGED=$?
echo "Protos changed: ${PROTOS_CHANGED}"

CURRENT_DATE=$(date '+%Y-%m-%d')

if [[ PROTOS_CHANGED -eq 1 ]]; then
  echo "Found changes in Google APIs to release"
  cd gcloud-sdk || exit
  git checkout -b update-protos-${CURRENT_DATE}
  git config user.name "GitHub Actions"
  git config user.email "<>"
  git commit -m "Google APIs updated at ${CURRENT_DATE}"
  git push origin update-protos-${CURRENT_DATE}
  gh pr create --title "Google APIs updated at ${CURRENT_DATE}" --fill
  cd ..
else
  echo "No changes are found in protos to release"
fi
