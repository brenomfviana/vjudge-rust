# Winterim Backpacking Trip

## Description

This Winter we are going on a trip along the Appalachian Trail. The trail is a
continuous marked footpath that goes from Katahdin in Maine to Springer Mountain
in Georgia, a distance of about 2160 miles. Even though our trip will only
consider some part of the trail, it will be our first real backpacking
experience and an excellent opportunity to acquire winter camping skills.

Part of the experience is also the route planning of the trip. We have a list
of all possible campsites that we can use along the way and we want to do this
trip so that we only stop K nights to camp. We also know in advance the
distance between consecutive campsites and we are only allowed to camp at a
campsite. Our goal is to plan the trip so that we minimise the maximum amount
of walking done in a single day. In other words, if our trip involves 2 nights
(3 days of walking), and we walk 9, 10, 5 miles on each day respectively, the
cost (maximum amount of walking done in one day) is 10. Another schedule that
involves walking 9, 6, 9 miles on each day has cost 9.

Given the distances between N consecutive campsites of a trail and given the
number of nights for your trip, K, your task is to devise a camping strategy
for the specified trail such that it minimises the maximum amount of walking
done in a single day. Note that the first distance value given is the distance
from our start-point of the trail to our 1st campsite, and the last distance
value given is the distance from our N-th campsite to our end-point of the
trail.


## Input

The input file contains several test cases, each of them as describes below.
The first line of input consists of two numbers, the number of campsites
(0 < N ≤ 600) and the number of nights of the trip (0 ≤ K ≤ 300). The following
N + 1 input lines indicate the distance in miles between consecutive campsite
locations.

### Sample Input

```
4 3
7
2
6
4
5
```


## Output

For each test case, on a line by itself, your program must output the maximum
amount of walking in a single day for the route that minimises such value.

### Sample Output

```
8
```
