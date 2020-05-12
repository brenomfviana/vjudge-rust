# Problem K: Through the Desert

## Description

Imagine you are an explorer trying to cross a desert. Many dangers and
obstacles are waiting on your path. Your life depends on your trusty old jeep
having a large enough fuel tank. But how large exactly does it have to be?
Compute the smallest volume needed to reach your goal on the other side.
The following events describe your journey:

- Fuel consumption n means that your truck needs n litres of gasoline per 100
km. n is an integer in the range [1..30]. Fuel consumption may change during
your journey, for example when you are driving up or down a mountain.

- Leak means that your truck's fuel tank was punctured by a sharp object. The
tank will start leaking gasoline at a rate of 1 litre of fuel per kilometre.
Multiple leaks add up and cause the truck to lose fuel at a faster rate.

However, not all events are troublesome in this desert. The following events
increase your chances of survival:

- Gas station lets you fill up your tank.

- Mechanic fixes all the leaks in your tank.

And finally:

- Goal means that you have safely reached the end of your journey!


## Input

The input consists of a series of test cases. Each test case consists of at
most 50 events. Each event is described by an integer, the distance (in
kilometres measured from the start) where the event happens, followed by the
type of event as above.

In each test case, the first event is of the form '0 Fuel consumption n', and
the last event is of the form 'd Goal' (d is the distance to the goal).

Events are given in sorted order by non-decreasing distance from the start, and
the 'Goal' event is always the last one. There may be multiple events at the
same distance—process them in the order given.

Input is terminated by a line containing '0 Fuel consumption 0' which should
not be processed.

### Sample Input

```
0 Fuel consumption 10
100 Goal
0 Fuel consumption 5
100 Fuel consumption 30
200 Goal
0 Fuel consumption 20
10 Leak
25 Leak
25 Fuel consumption 30
50 Gas station
70 Mechanic
100 Leak
120 Goal
0 Fuel consumption 0
```


## Output

For each test case, print a line containing the smallest possible tank volume
(in litres, with three digits precision after the decimal point) that will
guarantee a successful journey.

### Sample Output

```
10.000
35.000
81.000
```
