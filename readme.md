# huniq version 2

Command line utility to remove duplicates from the given input.
Note that huniq does not sort the input, it just removes duplicates.

```
SYNOPSIS: huniq -h # Shows help
SYNOPSIS: huniq [-c|--count] [-0|--null|-d DELIM|--delim DELIM]
```

```
$ echo -e "foo\nbar\nfoo\nbaz" | huniq
foo
bar
baz

$ echo -e "foo\nbar\nfoo\nbaz" | huniq -c
1 baz
1 bar
2 foo
```

`huniq` replaces `sort | uniq` (or `sort -u` with gnu sort) and `huniq -c` replaces `sort | uniq -c`.

The order of the output is stable when in normal mode, but it is not stable when
in -c/count mode.

## Installation

```
$ cargo install huniq
```

## Motivation

Sorting is slow. By using hash tables/hash sets instead of sorting
the input huniq is generally faster than `sort -u` or `sort | uniq -c` when testing with gnu sort/gnu uniq.

## Version History

Version 1 can be found [here](https://github.com/SoftwearDevelopment/huniq).

Changes made in version 2:

* The -d/-0 flags where added so you can specify custom delimiters
* Completely rewritten in rust.
* Version two is (depending on which benchmark you look at below) between 3.5x and 6.5x faster than version 1

## Build

```sh
cargo build --release
```

To run the tests execute:

```sh
bash ./test.sh
```

## Benchmark

You can use `bash ./benchmark.sh` to execute the benchmarks. They will execute until you manually abort them (e.g. by pressing Ctrl-C).

The benchmarks work by repeatedly feeding the implementations with data
from /usr/share/dict/* and measuring memory usage and time needed to process
the data with the unix `time` tool.

For the `uniq` algorithm, the results are posted below: We can see that the
rust implementation blows pretty much anything else out of the water in terms
of performance. Use sort only if you really need a coffee break, because you
won't get it with huniq! It beats the C++ implementation by a factor
of between 6.5 (for very few duplicates) and 3.5 (around 98% duplicates).
Compared to `sort -u`: huniq is around 30 times faster.

If memory efficiency is what you are looking for, use datamash which is not as fast as huniq
but uses the least memory (by a factor of around 3); failing that use `sort|uniq` which is a
lot slower but uses just very slightly more memory than datamash.

```
repetitions  implementation  seconds  memory/kb
1            huniq2-rust        0.26      29524
1            huniq1-c++         1.67      26188
1            awk                1.63     321936
1            datamash           1.78       9644
1            shell              7.30       9736
2            huniq2-rust        0.84      29592
2            huniq1-c++         3.28      26180
2            awk                3.71     322012
2            datamash           4.60       9636
2            shell             16.68       9740
5            huniq2-rust        2.02      29648
5            huniq1-c++         6.21      26184
5            awk                7.69     322012
5            datamash           9.10       9992
5            shell             44.71      10184
10           huniq2-rust        3.40      29676
10           huniq1-c++        12.84      26172
10           awk               16.73     321940
10           datamash          24.44      10032
10           shell             93.75      10036
50           huniq2-rust       14.68      29612
50           huniq1-c++        55.32      26200
50           awk               74.91     321940
50           datamash         103.54      10936
50           shell            453.94      10956
100          huniq2-rust       43.65      29492
100          huniq1-c++       154.99      26180
100          awk              239.66     321956
100          datamash         285.94      12148
100          shell           1062.07      12208
```

For the counting `huniq -c` implementation, the speed advantage
was less pronounced: Here the rust implementation is between 25%
and 50% faster than the C++ implementation and between 5x and 10x
faster than `sort | uniq -c`.

The increased memory usage of the rust implementation is much worse though:
The rust implementation needs about 2.2x more memory than the C++ implementation
and between 10x and 12x more memory than `sort | uniq`.

```
repetitions  implemetation  seconds  memory/kb
1            huniq2-rust       1.47     132096
1            huniq1-c++        1.85      60196
1            awk               2.79     362940
1            datamash          2.28       9636
1            shell             7.71      11716
2            huniq2-rust       2.32     132052
2            huniq1-c++        2.98      60156
2            awk               4.65     363016
2            datamash          5.27       9732
2            shell            16.37      11680
5            huniq2-rust       4.98     132092
5            huniq1-c++        7.54      60128
5            awk               9.37     363016
5            datamash         11.22       9964
5            shell            44.77      11948
10           huniq2-rust       8.81     132048
10           huniq1-c++       13.55      60196
10           awk              16.19     363032
10           datamash         25.12       9908
10           shell            90.01      11976
50           huniq2-rust      45.89     132092
50           huniq1-c++       74.04      60104
50           awk              85.43     362956
50           datamash        141.90      10996
50           shell           454.42      12876
100          huniq2-rust      90.80     132080
100          huniq1-c++      150.41      60196
100          awk             163.13     363008
100          datamash        322.70      12212
100          shell           933.67      14100
```

## Future direction

Feature wise huniq is pretty much complete, but the performance and memory usage should be improved in the future.

This first of all involves a better benchmarking setup which will probably consist
of an extra rust application that will use RNGs to generate test data for huniq and
take parameters like the number of elements to create, the rate of duplicates (0-1)
the length of strings to output and so on…

Then based on the improved benchmarking capabilities, some optimizations should be tried
like short string optimization, arena allocation, different hash functions, using
memory optimized hash tables, using an identity function for the `uniq` function
(we already feed it with hashes, so a second round of hashing is not necessary).

## License

Copyright © (C) 2020, Karolin Varner. All rights reserved.

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

    Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
    Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
    Neither the name of the Karolin Varner nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL Softwear, BV BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
