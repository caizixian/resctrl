# resctrl Agent

This is a wrapper shared library for putting a JVM into a COS (class of service)
using the Linux resctrl interface.
The shared library is controlled by two environment variables, `RESCTRL_COS` and `RESCTRL_SCHEMATA`;

The library will operate under `/sys/fs/resctrl/$RESCTRL_COS`.
If the current process is `java` (i.e., `argv[0]` ends with `java`), then the PID will be written to
`/sys/fs/resctrl/$RESCTRL_COS/tasks`, and all child threads will be added to the COS automatically.
Otherise, the agent will do nothing.

If you set `RESCTRL_SCHEMATA`, then the content will be written to `/sys/fs/resctrl/$RESCTRL_COS/schemata`.
Otherwise, the current schemata is preserved.
 
When the process exits, the content of `/sys/fs/resctrl/$RESCTRL_COS/tasks` is cleared, if the agent is active (i.e., running with a JVM).

You need to make sure that the current user is the owner of `/sys/fs/resctrl/$RESCTRL_COS/tasks` and `/sys/fs/resctrl/$RESCTRL_COS/schemata`, so that these files can be written to.

# Example
## Setup
```bash
sudo mount -t resctrl resctrl /sys/fs/resctrl
cd /sys/fs/resctrl
sudo mkdir dacapo_cos
cd dacapo_cos
sudo chown zixianc:zixianc tasks
sudo chown zixianc:zixianc schemata
```

## Build
```
cargo build --release
```

## Running programs
```console
$ RESCTRL_COS="dacapo_cos" RESCTRL_SCHEMATA="L3:0=0000;1=0000" LD_PRELOAD=$PWD/target/release/libresctrl.so perf stat /usr/lib/jvm/temurin-
17-jdk-amd64/bin/java -jar /usr/share/benchmarks/dacapo/dacapo-23.9-RC3-chopin.jar h2 -n 3
Operating on COS dacapo_cos
Setting schemata to L3:0=0000;1=0000
Setting tasks to 1258882
Hello 1258882, schemata     MB:0=2048;1=2048
    L3:0=0000;1=0000
, tasks 1258882

Using scaled threading model. 32 processors detected, 32 threads used to drive the workload, in a possible range of [1,1024]
Version: h2 2.2.220 derby 10.15.2.0 (use -p to print nominal benchmark stats)
Preparing the H2 database...
TPCC configuration: scale: 16, terminals: 32, transactions: 100000
===== DaCapo 23.9-RC3-chopin h2 starting warmup 1 =====
Starting 100000 requests...
100%
Completed requests
        Stock level .............   3973 ( 4.0%)
        Order status by name ....   2361 ( 2.4%)
        Order status by ID ......   1609 ( 1.6%)
        Payment by name .........  25847 (25.8%)
        Payment by ID ...........  17220 (17.2%)
        Delivery schedule .......   4029 ( 4.0%)
        New order ...............  44509 (44.5%)
        New order rollback ......    452 ( 0.5%)
===== DaCapo 23.9-RC3-chopin h2 completed warmup 1 in 6878 msec =====
===== DaCapo simple tail latency: 50% 602 usec, 90% 5951 usec, 99% 18979 usec, 99.9% 35013 usec, 99.99% 55585 usec, max 89408 usec, measured over 100000 events =====
===== DaCapo metered tail latency: 50% 792915 usec, 90% 1039091 usec, 99% 1049864 usec, 99.9% 1058446 usec, 99.99% 1067496 usec, max 1085802 usec, measured over 100000 events =====
===== DaCapo 23.9-RC3-chopin h2 starting warmup 2 =====
Starting 100000 requests...
100%
Completed requests
        Stock level .............   3973 ( 4.0%)
        Order status by name ....   2361 ( 2.4%)
        Order status by ID ......   1609 ( 1.6%)
        Payment by name .........  25847 (25.8%)
        Payment by ID ...........  17220 (17.2%)
        Delivery schedule .......   4029 ( 4.0%)
        New order ...............  44509 (44.5%)
        New order rollback ......    452 ( 0.5%)
===== DaCapo 23.9-RC3-chopin h2 completed warmup 2 in 4718 msec =====
===== DaCapo simple tail latency: 50% 417 usec, 90% 3939 usec, 99% 13867 usec, 99.9% 25405 usec, 99.99% 35936 usec, max 48642 usec, measured over 100000 events =====
===== DaCapo metered tail latency: 50% 106652 usec, 90% 207804 usec, 99% 238194 usec, 99.9% 244516 usec, 99.99% 254887 usec, max 266923 usec, measured over 100000 events =====
===== DaCapo 23.9-RC3-chopin h2 starting =====
Starting 100000 requests...
100%
Completed requests
        Stock level .............   3973 ( 4.0%)
        Order status by name ....   2361 ( 2.4%)
        Order status by ID ......   1609 ( 1.6%)
        Payment by name .........  25847 (25.8%)
        Payment by ID ...........  17220 (17.2%)
        Delivery schedule .......   4029 ( 4.0%)
        New order ...............  44509 (44.5%)
        New order rollback ......    452 ( 0.5%)
===== DaCapo 23.9-RC3-chopin h2 PASSED in 4509 msec =====
===== DaCapo simple tail latency: 50% 382 usec, 90% 3622 usec, 99% 13813 usec, 99.9% 24198 usec, 99.99% 34614 usec, max 47028 usec, measured over 100000 events =====
===== DaCapo metered tail latency: 50% 665 usec, 90% 14654 usec, 99% 25503 usec, 99.9% 34197 usec, 99.99% 44125 usec, max 59935 usec, measured over 100000 events =====
Operating on COS dacapo_cos
Clearing tasks

 Performance counter stats for '/usr/lib/jvm/temurin-17-jdk-amd64/bin/java -jar /usr/share/benchmarks/dacapo/dacapo-23.9-RC3-chopin.jar h2 -n 3':

        379,003.50 msec task-clock                       #   13.186 CPUs utilized             
           150,277      context-switches                 #  396.506 /sec                      
            23,173      cpu-migrations                   #   61.142 /sec                      
         2,180,511      page-faults                      #    5.753 K/sec                     
 1,276,062,771,368      cycles                           #    3.367 GHz                         (83.29%)
     4,364,759,840      stalled-cycles-frontend          #    0.34% frontend cycles idle        (83.33%)
    82,311,638,243      stalled-cycles-backend           #    6.45% backend cycles idle         (83.39%)
   847,207,831,918      instructions                     #    0.66  insn per cycle            
                                                  #    0.10  stalled cycles per insn     (83.40%)
   162,966,950,783      branches                         #  429.988 M/sec                       (83.34%)
     2,309,449,633      branch-misses                    #    1.42% of all branches             (83.29%)

      28.742477886 seconds time elapsed

     368.652118000 seconds user
      10.141901000 seconds sys
```

```console
$ RESCTRL_COS="dacapo_cos" RESCTRL_SCHEMATA="L3:0=ffff;1=ffff" LD_PRELOAD=$PWD/target/release/libresctrl.so perf stat /usr/lib/jvm/temurin-
17-jdk-amd64/bin/java -jar /usr/share/benchmarks/dacapo/dacapo-23.9-RC3-chopin.jar h2 -n 3
Operating on COS dacapo_cos
Setting schemata to L3:0=ffff;1=ffff
Setting tasks to 1259428
Hello 1259428, schemata     MB:0=2048;1=2048
    L3:0=ffff;1=ffff
, tasks 1259428

Using scaled threading model. 32 processors detected, 32 threads used to drive the workload, in a possible range of [1,1024]
Version: h2 2.2.220 derby 10.15.2.0 (use -p to print nominal benchmark stats)
Preparing the H2 database...
TPCC configuration: scale: 16, terminals: 32, transactions: 100000
===== DaCapo 23.9-RC3-chopin h2 starting warmup 1 =====
Starting 100000 requests...
100%
Completed requests
        Stock level .............   3973 ( 4.0%)
        Order status by name ....   2361 ( 2.4%)
        Order status by ID ......   1609 ( 1.6%)
        Payment by name .........  25847 (25.8%)
        Payment by ID ...........  17220 (17.2%)
        Delivery schedule .......   4029 ( 4.0%)
        New order ...............  44509 (44.5%)
        New order rollback ......    452 ( 0.5%)
===== DaCapo 23.9-RC3-chopin h2 completed warmup 1 in 3585 msec =====
===== DaCapo simple tail latency: 50% 322 usec, 90% 3050 usec, 99% 10479 usec, 99.9% 21308 usec, 99.99% 30042 usec, max 54112 usec, measured over 100000 events =====
===== DaCapo metered tail latency: 50% 367590 usec, 90% 571691 usec, 99% 612931 usec, 99.9% 617580 usec, 99.99% 622906 usec, max 627753 usec, measured over 100000 events =====
===== DaCapo 23.9-RC3-chopin h2 starting warmup 2 =====
Starting 100000 requests...
100%
Completed requests
        Stock level .............   3973 ( 4.0%)
        Order status by name ....   2361 ( 2.4%)
        Order status by ID ......   1609 ( 1.6%)
        Payment by name .........  25847 (25.8%)
        Payment by ID ...........  17220 (17.2%)
        Delivery schedule .......   4029 ( 4.0%)
        New order ...............  44509 (44.5%)
        New order rollback ......    452 ( 0.5%)
===== DaCapo 23.9-RC3-chopin h2 completed warmup 2 in 2788 msec =====
===== DaCapo simple tail latency: 50% 251 usec, 90% 1939 usec, 99% 9236 usec, 99.9% 19570 usec, 99.99% 27703 usec, max 38695 usec, measured over 100000 events =====
===== DaCapo metered tail latency: 50% 9810 usec, 90% 35904 usec, 99% 44756 usec, 99.9% 50006 usec, 99.99% 56204 usec, max 64875 usec, measured over 100000 events =====
===== DaCapo 23.9-RC3-chopin h2 starting =====
Starting 100000 requests...
100%
Completed requests
        Stock level .............   3973 ( 4.0%)
        Order status by name ....   2361 ( 2.4%)
        Order status by ID ......   1609 ( 1.6%)
        Payment by name .........  25847 (25.8%)
        Payment by ID ...........  17220 (17.2%)
        Delivery schedule .......   4029 ( 4.0%)
        New order ...............  44509 (44.5%)
        New order rollback ......    452 ( 0.5%)
===== DaCapo 23.9-RC3-chopin h2 PASSED in 2788 msec =====
===== DaCapo simple tail latency: 50% 243 usec, 90% 1921 usec, 99% 9175 usec, 99.9% 19241 usec, 99.99% 29108 usec, max 34147 usec, measured over 100000 events =====
===== DaCapo metered tail latency: 50% 260 usec, 90% 2070 usec, 99% 9198 usec, 99.9% 19260 usec, 99.99% 29108 usec, max 34147 usec, measured over 100000 events =====
Operating on COS dacapo_cos
Clearing tasks

 Performance counter stats for '/usr/lib/jvm/temurin-17-jdk-amd64/bin/java -jar /usr/share/benchmarks/dacapo/dacapo-23.9-RC3-chopin.jar h2 -n 3':

        210,033.51 msec task-clock                       #   10.480 CPUs utilized             
           129,766      context-switches                 #  617.835 /sec                      
            17,137      cpu-migrations                   #   81.592 /sec                      
         2,693,834      page-faults                      #   12.826 K/sec                     
   708,207,359,243      cycles                           #    3.372 GHz                         (83.30%)
     3,712,326,657      stalled-cycles-frontend          #    0.52% frontend cycles idle        (83.37%)
    42,684,433,254      stalled-cycles-backend           #    6.03% backend cycles idle         (83.31%)
   834,079,372,035      instructions                     #    1.18  insn per cycle            
                                                  #    0.05  stalled cycles per insn     (83.50%)
   161,719,077,903      branches                         #  769.968 M/sec                       (83.27%)
     2,160,291,673      branch-misses                    #    1.34% of all branches             (83.30%)

      20.041644681 seconds time elapsed

     203.553157000 seconds user
       6.357287000 seconds sys
```