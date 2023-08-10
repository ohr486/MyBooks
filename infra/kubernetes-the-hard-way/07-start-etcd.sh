#!/bin/bash

echo ===== start etcd =====

# start etcd server

sudo systemctl daemon-reload
sudo systemctl enable etcd
sudo systemctl start etcd

