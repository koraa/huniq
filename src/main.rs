use ahash::RandomState as ARandomState;
use anyhow::{anyhow, Result};
use bstr::{io::BufReadExt, ByteSlice};
use clap::{Arg, Command};
use std::cmp::Ordering;
use std::collections::{hash_map, HashMap, HashSet};
use std::hash::BuildHasherDefault;
use std::hash::{BuildHasher, Hasher};
use std::io::{stdin, stdout, BufRead, Write};
use std::mem;
use std::{default::Default, slice};

/// A no-operation hasher. Used as part of the uniq implementation,
/// because in there we manually hash the data and just store the
/// hashes of the data in the hash set. No need to hash twice
#[derive(Default)]
struct IdentityHasher {
    off: u8,
    buf: [u8; 8],
}

impl Hasher for IdentityHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.off += (&mut self.buf[self.off as usize..])
            .write(bytes)
            .unwrap_or(0) as u8;
    }

    fn finish(&self) -> u64 {
        u64::from_ne_bytes(self.buf)
    }
}

/// Hash the given value with the given BuildHasher. Now.
fn hash<T: BuildHasher, U: std::hash::Hash + ?Sized>(build: &T, v: &U) -> u64 {
    let mut s = build.build_hasher();
    v.hash(&mut s);
    s.finish()
}

enum Sort {
    Ascending,
    Descending,
}

/// Remove duplicates from stdin and print to stdout, counting
/// the number of occurrences.
fn count_cmd(delim: u8, sort: Option<Sort>) -> Result<()> {
    let mut set = HashMap::<Vec<u8>, u64, ARandomState>::default();
    for line in stdin().lock().split(delim) {
        match set.entry(line?) {
            hash_map::Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
            hash_map::Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }

    let result = if let Some(sort) = sort {
        sort_and_print(delim, sort, &set)
    } else {
        print_out(delim, set.iter().map(|(k, v)| (k.as_slice(), *v)))
    };

    mem::forget(set); // app can now exit, so we don't need to wait for this memory to be freed piecemeal

    result
}

type DataAndCount<'a> = (&'a [u8], u64);

/// Sorts the lines by occurence, then prints them
// TODO: this could be done more efficiently by reusing the memory of the HashMap
fn sort_and_print(delim: u8, sort: Sort, set: &HashMap<Vec<u8>, u64, ARandomState>) -> Result<()> {
    let mut seq: Vec<DataAndCount> = set.iter().map(|(k, v)| (k.as_slice(), *v)).collect();

    let comparator: fn(&DataAndCount, &DataAndCount) -> Ordering = match sort {
        Sort::Ascending => |a, b| a.1.cmp(&b.1),
        Sort::Descending => |a, b| b.1.cmp(&a.1),
    };
    seq.as_mut_slice().sort_by(comparator);
    print_out(delim, seq)
}

/// Prints the sequence of counts and data items, separated by delim
fn print_out<'a, I>(delim: u8, data: I) -> Result<()>
where
    I: IntoIterator<Item = DataAndCount<'a>>,
{
    let out = stdout();
    let mut out = out.lock();
    for (line, count) in data {
        write!(out, "{} ", count)?;
        out.write_all(&line)?;
        out.write_all(slice::from_ref(&delim))?;
    }

    Ok(())
}

/// Remove duplicates from stdin and print to stdout.
fn uniq_cmd(delim: u8, include_trailing: bool) -> Result<()> {
    // Line processing/output ///////////////////////
    let out = stdout();
    let inp = stdin();
    let hasher = ARandomState::new();
    let mut out = out.lock();
    let mut set = HashSet::<u64, BuildHasherDefault<IdentityHasher>>::default();

    inp.lock().for_byte_record_with_terminator(delim, |line| {
        let tok = trim_end(line, delim);
        if set.insert(hash(&hasher, &tok)) {
            out.write_all(&line)?;

            if include_trailing && tok.len() == line.len() {
                out.write_all(&[delim])?;
            }
        }
        Ok(true)
    })?;

    mem::forget(set); // app can now exit, so we don't need to wait for this memory to be freed piecemeal

    Ok(())
}

fn trim_end(mut record: &[u8], delim: u8) -> &[u8] {
    if record.last_byte() == Some(delim) {
        record = &record[..record.len() - 1];
    }
    record
}

fn try_main() -> Result<()> {
    let mut argspec = Command::new("huniq")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Remove duplicates from stdin, using a hash table")
        .author("Karolin Varner <karo@cupdev.net)")
        .arg(
            Arg::new("count")
                .help("Output the amount of times a line was encountered")
                .long("count")
                .short('c'),
        )
        .arg(
            Arg::new("sort")
                .help("Sort output by the number of occurences, in ascending order")
                .long("sort")
                .short('s'),
        )
        .arg(
            Arg::new("sort-descending")
                .help("Order output by the number of occurences, in descending order")
                .long("sort-descending")
                .short('S'),
        )
        .arg(
            Arg::new("delimiter")
                .help("Which delimiter between elements to use. By default `\n` is used")
                .long("delimiter")
                .long("delim")
                .short('d')
                .takes_value(true)
                .default_value("\n")
                .validator(|v| match v.len() {
                    1 => Ok(()),
                    _ => Err(String::from(
                        "\
Only ascii characters are supported as delimiters. \
Use sed to turn your delimiter into zero bytes?

    $ echo -n \"1λ1λ2λ3\" | sed 's@λ@\x00@g' | huniq -0 | sed 's@\x00@λ@g'
    1λ2λ3λ",
                    )),
                }),
        )
        .arg(
            Arg::new("null")
                .help("Use the \\0 character as the record delimiter.")
                .long("null")
                .short('0')
                .conflicts_with("delimiter"),
        )
        .arg(
            Arg::new("no-trailing-delimiter")
                .help("Prevent adding a delimiter to the last record if missing")
                .long("no-trailing-delimiter")
                .short('t'),
        );

    let args = argspec.try_get_matches_from_mut(&mut std::env::args_os())?;

    let delim = match args.is_present("null") {
        true => b'\0',
        false => args.value_of("delimiter").unwrap().as_bytes()[0],
    };

    let sort = match (args.is_present("sort"), args.is_present("sort-descending")) {
        (true, true) => return Err(anyhow!("cannot specify both --sort and --sort-descending")),
        (true, false) => Some(Sort::Ascending),
        (false, true) => Some(Sort::Descending),
        (false, false) => None,
    };

    match args.is_present("count") || sort.is_some() {
        true => count_cmd(delim, sort),
        false => uniq_cmd(delim, !args.is_present("no-trailing-delimiter")),
    }
}

fn main() {
    if let Err(er) = try_main() {
        println!("{}", er);
    }
}
