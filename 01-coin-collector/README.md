# Coin Collector

## Description

Our dear Sultan is visiting a country where there are n different types of
coin. He wants to collect as many different types of coin as you can. Now if he
wants to withdraw x amount of money from a bank, the bank will give him this
money using following algorithm.

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

First line of the input contains t the number of test cases. Each of the test
cases starts with n, where (1 <= n <= 1000), the number of different types of
coin. Next line contains n integers C_1, C_2, ..., C_n the value of each coin
type, where C_1 < C_2 < C_3 ... < C_n < 1000000000. C_1 equals to 1.

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
