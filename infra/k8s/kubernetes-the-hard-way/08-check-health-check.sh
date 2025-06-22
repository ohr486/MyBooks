#!/bin/bash

# cluster-info check
kubectl cluster-info --kubeconfig admin.kubeconfig

# health check
curl -H "Host: kubernetes.default.svc.cluster.local" -i http://127.0.0.1/healthz
