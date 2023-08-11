#!/bin/bash

for instance in controller-0 controller-1 controller-2; do
  gcloud compute scp 07-setup-etcd.sh 07-start-etcd.sh 07-check-etcd.sh ${instance}:~/
done

for instance in controller-0 controller-1 controller-2; do
  echo === ${instance}: 07-setup-etcd.sh ===
  gcloud compute ssh ${instance} --command="./07-setup-etcd.sh"
done

for instance in controller-0 controller-1 controller-2; do
  echo === ${instance}: 07-start-etcd.sh ===
  gcloud compute ssh ${instance} --command="./07-start-etcd.sh"
done

for instance in controller-0 controller-1 controller-2; do
  echo === ${instance}: 07-check-etcd.sh ===
  gcloud compute ssh ${instance} --command="./07-check-etcd.sh"
done
