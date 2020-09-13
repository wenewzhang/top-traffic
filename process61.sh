#!/bin/sh
scp root@154.17.2.61:/var/log/traffic.log .
cat traffic.log |awk '{ print $1,$2,$6,$7 }' > traffic-reduce.log
cargo run traffic-reduce.log
