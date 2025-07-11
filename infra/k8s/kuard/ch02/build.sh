#!/bin/bash

PROJECT="ohr486"
REPO="kuar-demo"
LOCATION="asia-northeast1"
IMAGE=$LOCATION-docker.pkg.dev/$PROJECT/$REPO/kuard-amd64:1

echo "--- copy kuard ---"
#cp ~/Git/github.com/ohr486/kuard/bin/blue/amd64/kuard bin/

echo "--- docker build ---"
#docker build -t kuard-amd64:1 .

echo "--- tag image ---"
#docker tag kuard-amd64:1 $IMAGE

echo "--- push image ---"
#docker push $IMAGE

