# Process schedulig simulation
This is a simulation of two process management algorithms:
1) Round Robin
2) First-Come First-Serve
This program try to simulate real scheduling so, theres some script to do this
for example when you all process and enter exit program take all them and try to
give them to scheduler at their (Entry time * Quantum time) milli second after
scheduler starts its work.
So its not exact as we calculate on paper mathematically :))


An other things is you can change Quantum time and Btime by chang code
now the default is 400 and 400 respectively.

## Sample input
```Chose what algorithm prefer to use
1 for RR and 2 is for FCFS
Witch one do you want to use? Enter number: 
1
Now enter all processes as template: name total_time entry_time then exit at end
P0 6 2
P1 2 5
P2 8 1
P3 3 0
P4 4 4
exit
Processes sorted by entry time:
P3 0 3
P2 1 8
P0 2 6
P4 4 4
P1 5 2
Time is 0
Process P3 launched
Process P3 launched
Time is 1
Process P3 launched
Time is 2
Time is 3
Process P2 launched
Process P0 launched
Time is 4
Process P2 launched
Time is 5
Time is 6
Process P0 launched
Time is 7
Process P4 launched
Time is 8
Process P2 launched
Time is 9
Process P1 launched
Time is 10
Process P0 launched
Time is 11
Process P4 launched
Time is 12
Process P2 launched
Time is 13
Process P1 launched
Time is 14
Process P0 launched
Time is 15
Process P4 launched
Time is 16
Process P2 launched
Time is 17
Process P0 launched
Time is 18
Process P4 launched
Time is 19
Process P2 launched
Time is 20
Process P0 launched
Time is 21
Process P2 launched
Time is 22
Process P2 launched
Time is 23
Time is 24
P1 wait 444 milli sec and turnaround 3701
P3 wait 1 milli sec and turnaround 1224
P2 wait 416 milli sec and turnaround 8973
P0 wait 424 milli sec and turnaround 7755
P4 wait 435 milli sec and turnaround 6136
Base on Quantum_time, Average waiting time: 0.86, Average turnaround time: 13.8945
All process hase been ended.
```
