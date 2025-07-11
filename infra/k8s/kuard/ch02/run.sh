#!/bin/bash

PROJECT="ohr486"
REPO="kuar-demo"
LOCATION="asia-northeast1"
IMAGE=$LOCATION-docker.pkg.dev/$PROJECT/$REPO/kuard-amd64:1

echo "--- run docker ---"
docker run -d --name kuard --publish 8080:8080 \
	--memory 200m --memory-swap 1G --cpu-shares 1024 \
	$IMAGE

open http://localhost:8080
