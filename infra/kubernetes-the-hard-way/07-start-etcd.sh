#!/bin/bash

# start etcd server

sudo systemctl daemon-reload
sudo systemctl unmask etcd
sudo systemctl enable etcd
sudo systemctl stop etcd

nohup sudo systemctl start etcd &

