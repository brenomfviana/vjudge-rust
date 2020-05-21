# Luggage

## Description

Peter and his friends are on holiday, so they have decided to make a trip by
car to know the north of Spain. They are seven people and they think that two
cars are enough for their luggage.

It's time to leave... and a heap of suitcases are awaiting out of the cars. The
drivers disagree about which suitcase must be put into each boot, because
nobody wants one boot to carry more weight than the other one. Is it possible
that the two boots load with the same weight? (Obviously without unpacking the
suitcases!)

Consider m sets of numbers representing suitcases weights, you must decide for
each one, if it is possible to distribute the suitcases into the boots, and the
two boots weigh the same.

## Input

The first line of the input contains an integer, m, indicating the number of
test cases.

For each test case, there is a line containing n integers (1 ≤ n ≤ 20)
separated by single spaces. These integers are the weights of each suitcase.
The total sum of the weights of all the suitcases is less or equal to 200
kilograms.

### Sample Input

```
3
1 2 1 2 1
2 3 4 1 2 5 10 50 3 50
3 5 2 7 1 7 5 2 8 9 1 25 15 8 3 1 38 45 8 1
```


## Output

The output consists of m lines. The i-th line corresponds with the i-th set of
suitcases weight and contains the string 'YES' or 'NO', depending on the
possibility that the two boots load with the same weight for the respective
test case.

### Sample Output

```
NO
YES
YES
```
