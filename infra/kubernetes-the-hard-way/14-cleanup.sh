#!/bin/bash

# cert
rm -rf *.pem *.csr *.json *.kubeconfig *.yaml

# compute instances
gcloud -q compute instances delete \
  controller-0 controller-1 controller-2 \
  worker-0 worker-1 worker-2 \
  --zone $(gcloud config get-value compute/zone)

# networking




#gcloud -q compute firewall-rules delete kubernetes-the-hard-way-allow-nginx-service
gcloud -q compute firewall-rules delete kubernetes-the-hard-way-allow-internal
gcloud -q compute firewall-rules delete kubernetes-the-hard-way-allow-external
#gcloud -q compute firewall-rules delete kubernetes-the-hard-way-allow-health-check


#gcloud -q compute routes delete kubernetes-route-10-200-0-0-24
#gcloud -q compute routes delete kubernetes-route-10-200-1-0-24
#gcloud -q compute routes delete kubernetes-route-10-200-2-0-24
gcloud -q compute networks subnets delete kubernetes
gcloud -q compute networks delete kubernetes-the-hard-way

gcloud -q compute addresses delete kubernetes-the-hard-way --region $(gcloud config get-value compute/region)
