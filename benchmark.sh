#!/usr/bin/env bash

set -e
trap "exit" SIGINT SIGTERM # exit from loop

cd "$(dirname "$0")"
huniq2bin="./target/release/huniq"
huniq1dir="./target/benchmark/huniq1"
huniq1bin="${huniq1dir}/huniq"

measure() {
  env time -f'%e %M' "$@"
}

meas_datamash() {
    if [[ "$@" = "" ]]; then
      { measure datamash -s groupby 1 first 1 >/dev/null; } 2>&1
    else
      { measure datamash -s groupby 1 count 1 >/dev/null; } 2>&1
    fi
}

meas_awk() {
  local cmd="$1"; shift
  if [[ "$@" = "" ]]; then
    { measure "$cmd" '!visited[$0]++' >/dev/null; } 2>&1
  else
    {
      measure "$cmd" '
        {
          visited[$0]++;
        }
        END {
          for (k in visited)
            print(k, visited[k]);
        }' >/dev/null
    } 2>&1
  fi
}

meas_shell() {
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

meas_exe() {
  { measure "$@" >/dev/null; } 2>&1
}

bench() {
  local name="$1"; shift
  yes | head -n "$repeats" \
    | while read _; do cat /usr/share/dict/*; done \
    | "$@" ${modeargs[${mode}]} \
    | while read results; do echo "$mode $repeats $name $results"; done
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
        bench 'huniq2-rust' meas_exe "${huniq2bin}"
        bench 'huniq1-c++ ' meas_exe "${huniq1bin}"
        bench 'awk-sys    ' meas_awk awk
        if which gawk 2>/dev/null >/dev/null; then
          bench 'gawk       ' meas_awk gawk
        fi
        if which nawk 2>/dev/null >/dev/null; then
          bench 'nawk       ' meas_awk nawk
        fi
        if which datamash 2>/dev/null >/dev/null; then
          bench 'datamash   ' meas_datamash
        fi
        bench 'shell      ' meas_shell
        echo
      done
    done
  done
}

main
