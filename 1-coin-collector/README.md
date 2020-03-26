# Coin Collector

## Description

Our dear Sultan is visiting a country where there are
![equation](http://latex.codecogs.com/gif.latex?n) different types of
coin. He wants to collect as many different types of coin as you can. Now if he
wants to withdraw ![equation](http://latex.codecogs.com/gif.latex?x) amount of
money from a bank, the bank will give him this money using following algorithm.

```
withdraw(x) {
  if (x == 0) return;
  Let y be the highest valued coin that does not exceed x;
  Give the customer y valued coin;
  withdraw(x - y);
}
```

Now Sultan can withdraw any amount of money from the bank. He should maximize
the number of different coins that he can collect in a single withdrawal.


## Input

First line of the input contains
![equation](http://latex.codecogs.com/gif.latex?t) the number of test cases.
Each of the test cases starts with
![equation](http://latex.codecogs.com/gif.latex?n), where
![equation](http://latex.codecogs.com/gif.latex?%281%20%3C%3D%20n%20%3C%3D%201000%29),
the number of different types of coin. Next line contains
![equation](http://latex.codecogs.com/gif.latex?n) integers
![equation](http://latex.codecogs.com/gif.latex?C_1%2C%20C_2%2C%20...%2C%20C_n)
the value of each coin type , where
![equation](http://latex.codecogs.com/gif.latex?C_1%20%3C%20C_2%20%3C%20C_3%20%3C%20.%20.%20.%20%3C%20C_n%20%3C%201000000000).
![equation](http://latex.codecogs.com/gif.latex?C_1) equals to ![equation](http://latex.codecogs.com/gif.latex?1).

### Sample Input

```
2
6
1 2 4 8 16 32
6
1 3 6 8 15 20
```


## Output

For each test case output one line denoting the maximum number of coins that
Sultan can collect in a single withdrawal. He can withdraw infinite amount of
money from the bank.

### Sample Output

```
6
4
```
