#!/bin/bash

# start controller

sudo systemctl daemon-reload
sudo systemctl unmask kube-apiserver kube-controller-manager kube-scheduler
sudo systemctl enable kube-apiserver kube-controller-manager kube-scheduler
sudo systemctl stop kube-apiserver kube-controller-manager kube-scheduler

sudo systemctl start kube-apiserver kube-controller-manager kube-scheduler

