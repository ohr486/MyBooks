#!/bin/bash

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
