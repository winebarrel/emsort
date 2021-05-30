# emsort

Rust [external merge sort](https://en.wikipedia.org/wiki/External_sorting#External_merge_sort) implementation.

## Usage

```
% cargo run -- -h
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/emsort -h`
Usage: target/debug/emsort [OPTIONS] FILE

Options:
    -c, --capacity SIZE chunk capacity (default: 10485760)
    -v, --version       print version and exit
    -h, --help          print usage and exit
```

```
% cal 2021 > cal.txt
% cargo run -- cal.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/emsort cal.txt`




                            2021
                      30 31
                1  2      1  2  3  4  5  6            1  2  3  4
                1  2      1  2  3  4  5  6      1  2  3  4  5  6
             1  2  3                     1         1  2  3  4  5
             1  2  3   1  2  3  4  5  6  7            1  2  3  4
         1月                    2月                    3月
         4月                    5月                    6月
         7月                    8月                    9月
        10月                   11月                   12月
 3  4  5  6  7  8  9   7  8  9 10 11 12 13   5  6  7  8  9 10 11
 3  4  5  6  7  8  9   7  8  9 10 11 12 13   7  8  9 10 11 12 13
 4  5  6  7  8  9 10   2  3  4  5  6  7  8   6  7  8  9 10 11 12
 4  5  6  7  8  9 10   8  9 10 11 12 13 14   5  6  7  8  9 10 11
10 11 12 13 14 15 16  14 15 16 17 18 19 20  12 13 14 15 16 17 18
10 11 12 13 14 15 16  14 15 16 17 18 19 20  14 15 16 17 18 19 20
11 12 13 14 15 16 17   9 10 11 12 13 14 15  13 14 15 16 17 18 19
11 12 13 14 15 16 17  15 16 17 18 19 20 21  12 13 14 15 16 17 18
17 18 19 20 21 22 23  21 22 23 24 25 26 27  19 20 21 22 23 24 25
17 18 19 20 21 22 23  21 22 23 24 25 26 27  21 22 23 24 25 26 27
18 19 20 21 22 23 24  16 17 18 19 20 21 22  20 21 22 23 24 25 26
18 19 20 21 22 23 24  22 23 24 25 26 27 28  19 20 21 22 23 24 25
24 25 26 27 28 29 30  28                    28 29 30 31
24 25 26 27 28 29 30  28 29 30              26 27 28 29 30 31
25 26 27 28 29 30     23 24 25 26 27 28 29  27 28 29 30
25 26 27 28 29 30 31  29 30 31              26 27 28 29 30
31
31
日 月 火 水 木 金 土  日 月 火 水 木 金 土  日 月 火 水 木 金 土
日 月 火 水 木 金 土  日 月 火 水 木 金 土  日 月 火 水 木 金 土
日 月 火 水 木 金 土  日 月 火 水 木 金 土  日 月 火 水 木 金 土
日 月 火 水 木 金 土  日 月 火 水 木 金 土  日 月 火 水 木 金 土
```
