# huniq version 2

Command line utility to remove duplicates from the given input.

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

`huniq` replaces `sort | uniq` and `huniq -c` replaces `sort | uniq -c`.

The order of the output is stable when in normal mode, but it is not stable when
in -c/count mode.

## Installation

```
$ cargo install huniq
```

## Motivation

Sorting is slow. By using hash tables/hash sets instead of sorting
the input huniq is generally faster than `sort -u` or `sort | uniq -c`.

## Version History

Version 1 can be found [here](https://github.com/SoftwearDevelopment/huniq).

Changes made in version 2:

* The -d/-0 flags where added so you can specify custom delimiters
* Completely rewritten in rust.
* Version two is (depending on which benchmark you look at below) between 1.25 and 3.6x faster than version 1

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
rust implementation is the very fastest. It beats the C++ implementation by a factor
of between 3.6 (for very few duplicates) and 1.7 (around 98% duplicates).
The difference is even starker when compared to `sort -u`: huniq is between 12 and 50 times faster.

Surprisingly, `uniq -u` was the most memory efficient. It beat both the rust and
C++ implementation by a factor of between 2.7 and 3. The Rust implementation
has a slightly worse memory footprint than the C++ one. It uses around 14%
more memory.

```
repetitions  implementation  seconds  memory/kb
1            rust               0.57      29648
1            cpp                2.05      26092
1            shell              8.62       9932
2            rust               1.54      29616
2            cpp                4.47      26060
2            shell             23.99       9932
5            rust               4.56      29512
5            cpp                7.45      26116
5            shell             50.88       9996
10           rust              11.54      29512
10           cpp               16.33      26144
10           shell            101.04      10156
50           rust              34.13      29632
50           cpp               58.62      26112
50           shell            421.27      10884
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
1            rust              1.31     132096
1            cpp               1.65      60068
1            shell             7.09      11500
2            rust              1.95     132064
2            cpp               2.73      60076
2            shell            13.55      11792
5            rust              4.16     132220
5            cpp               5.80      60152
5            shell            36.35      11988
10           rust              8.12     132104
10           cpp              11.02      60128
10           shell            72.01      11984
50           rust             36.15     132100
50           cpp              54.13      60052
50           shell           356.69      13136
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
