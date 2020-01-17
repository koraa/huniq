#! /bin/sh

set -e
trap "exit" SIGINT SIGTERM # exit from loop

cd "$(dirname "$0")"
huniq2bin="./target/release/huniq"
huniq1dir="./target/benchmark/huniq1"
huniq1bin="${huniq1dir}/huniq"

measure() {
  env time -f'%e %M' "$@"
}

bench_rust() {
  { measure ./target/release/huniq "$@" >/dev/null; } 2>&1
}

bench_cpp() {
  { measure "$huniq1bin" "$@" >/dev/null; } 2>&1
}

bench_shell() {
  if [[ "$@" = "" ]]; then
    { measure sort -u >/dev/null; } 2>&1
  else
    {
      measure sort | measure uniq -c >/dev/null
    } 2>&1 | awk '
      {
        elapsed=$1;
        mem+=$2;
      }

      END {
        print(elapsed, mem);
      }'
  fi
}

main() {
  test -e "$huniq2bin" || {
    cargo build --release
  }

  test -e "$huniq1dir" || {
    git clone --recursive "https://github.com/SoftwearDevelopment/huniq" "$huniq1dir"
  } >&2

  test -e "$huniq1bin" || {
    cd "$huniq1dir"
    make
    cd -
  } >&2

  declare -A modeargs
  modeargs[uniq]=""
  modeargs[count]="-c"

  while true; do
    for mode in "uniq" "count"; do
      for repeats in 1 2 5 10 50; do
        for impl in rust cpp shell; do
          yes | head -n "$repeats" \
            | while read _; do cat /usr/share/dict/*; done \
            | "bench_${impl}" ${modeargs[${mode}]} \
            | while read results; do echo "$mode $repeats $impl $results"; done
        done
      done
    done
  done
}

main
