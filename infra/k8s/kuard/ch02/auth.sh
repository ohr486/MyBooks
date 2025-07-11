#!/bin/bash

ACC="ohr486@gmail.com"
REPO="kuar-demo"
LOCATION="asia-northeast1"

echo "--- list gcloud auth ---"
#gcloud auth list

echo "--- set account to [ $ACC ] ---"
#gcloud config set account $ACC

echo "--- configure docker ---"
#gcloud auth configure-docker asia-northeast1-docker.pkg.dev

echo "--- create repository ---"
#gcloud artifacts repositories create $REPO --repository-format=docker --location=$LOCATION --description="Kuard Demo"
