https://github.com/wenewzhang/top-traffic.git
```
cat /var/log/traffic.log |awk '{ print $1,$6,$7 }'
```

capture the traffic every 3 minutes:
```
*/3 * * * *     /usr/sbin/iftop -t -s 30 >> /var/log/traffic.log
```
