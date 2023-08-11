#!/bin/bash

for instance in controller-0 controller-1 controller-2; do
  gcloud compute scp 08-setup-controller.sh 08-start-controller.sh ${instance}:~/
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
  echo ${instance}
done
