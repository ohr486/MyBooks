#!/bin/bash

# configure network bridge

POD_CIDR=$(curl -s -H "Metadata-Flavor: Google" \
  http://metadata.google.internal/computeMetadata/v1/instance/attributes/pod-cidr)

cat <<EOF | sudo tee /etc/cni/net.d/10-bridge.conf
{
  "cniVersion": "0.4.0",
  "name": "bridge",
  "type": "bridge",
  "bridge": "cnio0",
  "isGateway": true,
  "ipam": {
    "type": "host-local",
    "ranges": [
      [{"subnet": "${POD_CIDR}"}]
    ],
    "routes": [{"dist": "0.0.0.0/0"}]
  }
}
EOF

# create loopback

cat <<EOF | sudo tee /etc/cni/net.d/99-loopback.conf
{
  "cniVersion": "0.4.0",
  "name": "lo",
  "type": "loopback"
}
EOF

