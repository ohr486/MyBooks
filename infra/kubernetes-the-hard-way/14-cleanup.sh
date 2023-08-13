#!/bin/bash

# delete files
rm -rf *.pem *.csr *.json *.kubeconfig *.yaml



# delete compute instances

gcloud -q compute instances delete \
  controller-0 controller-1 controller-2 \
  worker-0 worker-1 worker-2 \
  --zone $(gcloud config get-value compute/zone)



# delete network resources

gcloud -q compute forwarding-rules delete kubernetes-forwarding-rule \
  --region $(gcloud config get-value compute/region)
gcloud -q compute target-pools delete kubernetes-target-pool
gcloud -q compute http-health-checks delete kubernetes
gcloud -q compute addresses delete kubernetes-the-hard-way



# delete firewall rules

#gcloud -q compute firewall-rules delete kubernetes-the-hard-way-allow-nginx-service
gcloud -q compute firewall-rules delete kubernetes-the-hard-way-allow-internal
gcloud -q compute firewall-rules delete kubernetes-the-hard-way-allow-external
gcloud -q compute firewall-rules delete kubernetes-the-hard-way-allow-health-check



# delete vpc

#gcloud -q compute routes delete kubernetes-route-10-200-0-0-24
#gcloud -q compute routes delete kubernetes-route-10-200-1-0-24
#gcloud -q compute routes delete kubernetes-route-10-200-2-0-24
gcloud -q compute networks subnets delete kubernetes
gcloud -q compute networks delete kubernetes-the-hard-way



# delete compute address

gcloud -q compute addresses delete kubernetes-the-hard-way \
  --region $(gcloud config get-value compute/region)

