https://github.com/wenewzhang/top-traffic.git
```
cat traffic.log |awk '{ print $1,$2,$6,$7 }' > traffic-reduce.log
```

capture the traffic every 3 minutes:
```
*/3 * * * *     /usr/sbin/iftop -t -s 30 >> /var/log/traffic.log
```
