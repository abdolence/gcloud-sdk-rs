#!/bin/bash

cd gcloud-sdk
cargo release --package gcloud-sdk patch --no-dev-version --dry-run
cd ..
