#!/bin/sh
scp -P 80 root@144.34.177.45:/var/log/traffic.log .
cat traffic.log |awk '{ print $1,$2,$6,$7 }' > traffic-reduce.log
cargo run traffic-reduce.log
