# VJudge Rust

## Description

This repository contains Rust solutions of some CodeChef and UVa problems.
These problems were homeworks for the Algorithms Project course of University
of São Paulo (USP).

When I code these solutions there was no option to send Rust code to VJudge
(the official code judge of our class) or another code judging system
(**Are we judged yet?**). I also did not find any Rust solution for these
problems. I solved the course homeworks by using C++, however I wanted to
improve my Rust skills and develop Rust solutions could be not trivial (and
were not) for some problems. Therefore, I decided to recode these homeworks by
using Rust to learn the Rust way to solve problems. I have made a lot of code
refactoring as you can see on the commits list. My first implementations did
not have a clean code and there were a lot of improvements to make. I just did
these improvements after the end of my course and I maybe do improve a little
more sometime later.


### Some observations

- Rust code, in some situations, can be very verbose due the use of `Option`, `Result` and `HashMap`;
- Rust code can be very clean, we can do a lot of things with little code;
- Reverse sort could be simpler (something like `rev_sort()`) not all of this: `sort_by_key(|&num| Reverse(num))`
- C++ HashMap is very different of Rust HashMap. The first one is really flexible, but can lead to unpredictable behaviors because an element can be created just by calling for a not added key (`hmap[i]`). Rust HashMap is more strict because the borrowing rules.


### Some things that I missed

- Tuple assignment for already defined variables, something like:
  ```
  let (mut a, mut b) = (0, 0);
  (a, b) = (4, 2);
  ```
  Maybe there is a reason for Rust do not allow this, but it would make the code cleaner.
- The `do...while` syntax sugar. I prefer use this:
  ```
  do {
    a += 1;
  } while a > b;
  ```
  Instead of this:
  ```
  loop {
    a += 1;
    if a > b { break }
  }
  ```
  Maybe we also could have a `do...while let`, but I cannot think in an example of usage of this.
- A function to read char by char from an input (`read_char(&mut char)`):
  ```
  let mut input = char::new();
  io::stdin().read_char(&mut input);
  ```
  Use this function is better to use for some problems, like the last one of this repository.


## List of Problems

| Problem | Online Judge | Code |
|---|:-:|:-:|
| 00-stable-marriage-problem    | CodeChef | [STABLEMP](https://www.codechef.com/problems/STABLEMP) |
| 01-coin-collector             | UVa      | [11264](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=2231) |
| 02-scarecrow                  | UVa      | [12405](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=3836) |
| 03-match-making-problem       | UVa      | [12210](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=3362) |
| 04-backt-to-the-8-queens      | UVa      | [11085](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=2026) |
| 05-sum-it-up                  | UVa      | [574](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=515) |
| 06-rat-in-a-maze              | CodeChef | [MM1803](https://www.codechef.com/problems/MM1803) |
| 07-knuths-permutation         | UVa      | [10063](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=1004) |
| 08-9puzzle                    | UVa      | [11513](https://onlinejudge.org/index.php?option=onlinejudge&page=show_problem&problem=2508) |
| 09-15puzzle                   | UVa      | [10181](https://onlinejudge.org/index.php?option=onlinejudge&Itemid=8&page=show_problem&problem=1122) |
| 10-inversions                 |    --    |  --  |
| 11-solve-it                   | UVa      | [10341](https://onlinejudge.org/index.php?option=onlinejudge&page=show_problem&problem=1282) |
| 12-problemk                   | UVa      | [11935](https://onlinejudge.org/index.php?option=onlinejudge&page=show_problem&problem=3086) |
| 13-winterim-backpacking-trip  | UVa      | [907](https://onlinejudge.org/index.php?option=onlinejudge&page=show_problem&problem=848) |
| 14-luggage                    | UVa      | [10664](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=1605) |
| 15-supersale                  | UVa      | [10130](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=13&page=show_problem&problem=10712) |
| 16-is-bigger-smarter          | UVa      | [10131](https://onlinejudge.org/index.php?option=onlinejudge&Itemid=8&page=show_problem&problem=1072) |
| 17-let-me-count-the-ways      | UVa      | [357](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=24&page=show_problem&problem=293) |
| 18-rare-order                 | UVa      | [200](https://onlinejudge.org/index.php?option=onlinejudge&page=show_problem&problem=136) |
| 19-counting-stars             | UVa      | [11244](https://onlinejudge.org/index.php?option=onlinejudge&Itemid=8&page=show_problem&problem=2201) |
| 20-bicoloring                 | UVa      | [10004](https://onlinejudge.org/index.php?option=onlinejudge&Itemid=8&page=show_problem&problem=945) |
| 21-maze-traversal             | UVa      | [10377](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=1318) |
| 22-racing                     | UVa      | [1234](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=3675) |
| 23-sending-email              | UVa      | [10986](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=1927) |
| 24-wormholes                  | UVa      | [558](https://onlinejudge.org/index.php?option=onlinejudge&Itemid=8&page=show_problem&problem=499) |
| 25-page-hopping               | UVa      | [821](https://onlinejudge.org/index.php?option=onlinejudge&page=show_problem&problem=762) |
| 26-sabotage                   | UVa      | [10480](https://onlinejudge.org/index.php?option=onlinejudge&page=show_problem&problem=1421) |
| 27-factors-and-factorials     | UVa      | [160](https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=96) |
| 28-ocean-deep-make-it-shallow | UVa      | [10176](https://onlinejudge.org/index.php?option=onlinejudge&Itemid=8&page=show_problem&problem=1117) |

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE)
file for details.
