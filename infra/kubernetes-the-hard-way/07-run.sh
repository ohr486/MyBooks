#!/bin/bash

for instance in controller-0 controller-1 controller-2; do
  gcloud compute scp 07-configure-etcd.sh 07-start-etcd.sh 07-check-etcd.sh ${instance}:~/
done

for instance in controller-0 controller-1 controller-2; do
  gcloud compute ssh ${instance} --command="./07-configure-etcd.sh"
done

for instance in controller-0 controller-1 controller-2; do
  gcloud compute ssh ${instance} --command="./07-start-etcd.sh"
done

for instance in controller-0 controller-1 controller-2; do
  gcloud compute ssh ${instance} --command="./07-check-etcd.sh"
done
