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

Sorting is slow and requires lot's of memory. By using hash tables/hash sets instead of sorting
the input huniq is generally faster and requires less memory than the combination of `sort` and `uniq.`

## Version History

Version 1 can be found [here](https://github.com/SoftwearDevelopment/huniq).

Changes made in version 2:

* The -d/-0 flags where added so you can specify custom delimiters
* Completely rewritten in rust.

## License

Copyright © (C) 2020, Karolin Varner. All rights reserved.

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

    Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
    Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
    Neither the name of the Karolin Varner nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL Softwear, BV BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
