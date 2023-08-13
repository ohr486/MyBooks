#!/bin/bash

for instance in worker-0 worker-1 worker-2; do
  gcloud compute scp 09-setup-worker.sh 09-configure-cni.sh ${instance}:~/
done

for instance in worker-0 worker-1 worker-2; do
  echo === ${instance}: 09-setup-worker.sh ===
  gcloud compute ssh ${instance} --command="./09-setup-worker.sh"
done

for instance in worker-0 worker-1 worker-2; do
  echo === ${instance}: 09-configure-cni.sh ===
  gcloud compute ssh ${instance} --command="./09-configure-cni.sh"
done

