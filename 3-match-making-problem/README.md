# A Match Making Problem

## Description

Match-making is a tough job and even then its long term success (A happy
family) depends on two people who are often not involved in the match making
process. But now, sites like Facebook, Twitter and communication devices/
software like mobile phones, messengers have made the professional
match makers jobless. So these angry and jobless match makers have gathered
together to make the government pass a law in the parliament that will stop
people from choosing their life partners. The law is stated below:

In a certain community, the most senior bachelor must marry a spinster (Female
Bachelor) whose age is nearest to him. Then next senior bachelor will then
marry a spinster whose age is nearest to him (of course if there is a tie,
marrying anyone of them will do) excluding the spinster that has already got
married. This process continues until there is no bachelor or spinster left.
Of course a bachelor cannot marry two spinsters and a spinster cannot marry two
bachelors.

For example in a community there are four bachelors who are 21, 25, 26, 2 years
old and four spinsters who are 26, 24, 25 and 35 years old. The example below
shows the only possibility of marriage: (eg: The 26 year old bachelor marries
the 25 years old spinster). Now given the ages of the bachelors and the
spinsters in a community you will have to find the number of bachelors left,
after all the marriages have taken place according to the law mentioned above.
Also you have to report the age of the youngest bachelor left in the community
if there is one.

Example:
```
# Bachelors
[26, 25, 21, 2]
# Spinsters
[35, 25, 24, 23]

# Result
26 25
25 24
21 23
2  35
```


## Input

The input file contains at most 25 sets of inputs. The description of each set
is given below:

The first line of each set contains two integers B (0 < B < 10000) and
S (0 < S < 10000) which denotes the total number of bachelors and spinsters in
the community respectively. Each of the next B lines contains one integer
between 2 and 60 (inclusive) which denotes the age of one bachelor in the
community. Each of the next S lines contains one integer between 2 and 60
(inclusive) which denotes the age of one spinster in the community. For
simplicity you don't need to worry about getting married at a very small age in
this problem. That means unmarried people of all ages are valid bachelor or
spinster.

Input is terminated by a line containing two zeroes.

### Sample Input

4 4
26
25
2
21
35
25
23
24
1 2
20
30
40
4 2
5
5
10
15
20
18
0 0


## Output

For each set of input produce one line of output. This line contains the serial
of output followed by one or two integers. The first integer denotes the number
of bachelors left in the community after all potential marriages have been
completed. If this integer is not zero then print a second integer which
denotes the age of the youngest bachelor left in the community after all
possible marriages have been completed. Look at the output for sample input for
details.

### Sample Output

Case 1: 0
Case 2: 0
Case 3: 2 5
