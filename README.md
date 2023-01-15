# TODO: Write README

## Comparison after rewrite
Comparison between 55a0cc7 and 283f120. Code was re-written to read all markdown files once on startup instead on every request.

```bash
heiskane@heiskane-destop:~/code-stuff/markdown_cms (master *)$ ab -n 100000 -c 1000 http://localhost:8080/
This is ApacheBench, Version 2.3 <$Revision: 1843412 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 10000 requests
Completed 20000 requests
Completed 30000 requests
Completed 40000 requests
Completed 50000 requests
Completed 60000 requests
Completed 70000 requests
Completed 80000 requests
Completed 90000 requests
Completed 100000 requests
Finished 100000 requests


Server Software:
Server Hostname:        localhost
Server Port:            8080

Document Path:          /
Document Length:        628 bytes

Concurrency Level:      1000
Time taken for tests:   7.963 seconds
Complete requests:      100000
Failed requests:        0
Total transferred:      74500000 bytes
HTML transferred:       62800000 bytes
Requests per second:    12558.81 [#/sec] (mean)
Time per request:       79.625 [ms] (mean)
Time per request:       0.080 [ms] (mean, across all concurrent requests)
Transfer rate:          9137.02 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0   30   6.2     30      56
Processing:    12   49  10.3     48     129
Waiting:        2   38  10.2     38     114
Total:         31   79  11.3     79     146

Percentage of the requests served within a certain time (ms)
  50%     79
  66%     83
  75%     86
  80%     88
  90%     93
  95%     99
  98%    106
  99%    111
 100%    146 (longest request)
heiskane@heiskane-destop:~/code-stuff/markdown_cms (master *)$
heiskane@heiskane-destop:~/code-stuff/markdown_cms ((283f120...))$
heiskane@heiskane-destop:~/code-stuff/markdown_cms ((283f120...))$ ab -n 100000 -c 1000 http://localhost:8080/
This is ApacheBench, Version 2.3 <$Revision: 1843412 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 10000 requests
Completed 20000 requests
Completed 30000 requests
Completed 40000 requests
Completed 50000 requests
Completed 60000 requests
Completed 70000 requests
Completed 80000 requests
Completed 90000 requests
Completed 100000 requests
Finished 100000 requests


Server Software:
Server Hostname:        localhost
Server Port:            8080

Document Path:          /
Document Length:        47 bytes

Concurrency Level:      1000
Time taken for tests:   50.249 seconds
Complete requests:      100000
Failed requests:        0
Total transferred:      16300000 bytes
HTML transferred:       4700000 bytes
Requests per second:    1990.11 [#/sec] (mean)
Time per request:       502.486 [ms] (mean)
Time per request:       0.502 [ms] (mean, across all concurrent requests)
Transfer rate:          316.78 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    5   4.6      4      31
Processing:    10  496  74.3    498    1010
Waiting:        1  494  74.1    496    1005
Total:         29  501  72.7    504    1011

Percentage of the requests served within a certain time (ms)
  50%    504
  66%    513
  75%    521
  80%    530
  90%    548
  95%    555
  98%    562
  99%    567
 100%   1011 (longest request)
```
