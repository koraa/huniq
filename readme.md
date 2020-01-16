# huniq 2

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

## Motivation

Sorting is slow. By using hash tables/hash sets instead of sorting
the input huniq is generally faster than `sort -u` or `sort | uniq -c`.

## Version History

Version 1 can be found [here](https://github.com/SoftwearDevelopment/huniq).

Changes made in version 2:

* The -d/-0 flags where added so you can specify custom delimiters
* Completely rewritten in rust.
* Version two was in the (admittedly very limited) between 1.4x 

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

From my very limited benchmarking results, I found that the rust implementation is between 1-2x faster than the C++ implementation and between 5-10 times
faster than the `uniq/sort` standard implementation.

Surprisingly, sort features the lowest memory usage. All three implementations' memory usage grow with the number of unique lines, and not the number
of total lines, so sort probably manually optimizes for that. Sort's memory usage us about a third of the rust implementation…
The difference in memory usage between the rust implementation and the C++ one is quite small; the C++ one uses around 10% less memory.

The benchmark 

```
repetitions  implemetation  seconds  memory/kb
1            rust              0.89      29568
1            cpp               2.62      26080
1            shell            10.21       9820
2            rust              2.02      29604
2            cpp               6.21      26036
2            shell            34.33       9724
5            rust              5.25      29548
5            cpp              12.08      26076
5            shell            72.30      10004
10           rust             10.26      29548
10           cpp              18.87      26128
10           shell           151.40      10060
50           rust             50.16      29548
50           cpp              88.51      26096
50           shell           675.88      11048
100          rust             84.96      29604
100          cpp             149.10      26048
```

## Future direction

There is some potential for optimizations…like

## License

Copyright © (C) 2020, Karolin Varner. All rights reserved.

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

    Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
    Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
    Neither the name of the Karolin Varner nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL Softwear, BV BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
