#!/bin/bash

for instance in controller-0 controller-1 controller-2; do
  gcloud compute scp 08-setup-controller.sh 08-start-controller.sh \
    08-setup-health-check.sh 08-check-health-check.sh 08-rbac-for-auth.sh ${instance}:~/
done

for instance in controller-0 controller-1 controller-2; do
  echo === ${instance}: 08-setup-controller.sh ===
  gcloud compute ssh ${instance} --command="./08-setup-controller.sh"
done

for instance in controller-0 controller-1 controller-2; do
  echo === ${instance}: 08-start-controller.sh ===
  gcloud compute ssh ${instance} --command="./08-start-controller.sh"
done

for instance in controller-0 controller-1 controller-2; do
  echo === ${instance}: 08-setup-health-check.sh ===
  gcloud compute ssh ${instance} --command="./08-setup-health-check.sh"
done

for instance in controller-0 controller-1 controller-2; do
  echo === ${instance}: 08-check-health-check.sh ===
  gcloud compute ssh ${instance} --command="./08-check-health-check.sh"
done

# run at only controller-0
for instance in controller-0; do
  echo === ${instance}: 08-rbac-for-auth.sh ===
  gcloud compute ssh ${instance} --command="./08-rbac-for-auth.sh"
done

./08-k8s-frontend-lb.sh
