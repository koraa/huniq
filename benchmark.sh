#! /bin/sh

set -e
trap "exit" SIGINT SIGTERM # exit from loop

cd "$(dirname "$0")"
huniq2bin="./target/release/huniq"
huniq1dir="./target/benchmark/huniq1"
huniq1bin="${huniq1dir}/huniq"

measure() {
  env time -f'%e %M' "$@" >/dev/null
}

bench_rust() {
  measure ./target/release/huniq "$@"
}

bench_cpp() {
  measure "$huniq1bin" "$@"
}

bench_shell() {
  if [[ "$@" = "" ]]; then
    measure sort -u
  else
    {
      measure sort | measure uniq
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
      for repeats in 1 2 5 10 50 100; do
        for impl in rust cpp shell; do
          echo -n "$mode $repeats $impl "
          yes | head -n "$repeats" | while read _; do cat /usr/share/dict/*; done \
            | "bench_${impl}" ${modeargs[${mode}]}
        done
      done
    done
  done
}
