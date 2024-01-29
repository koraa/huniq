use anyhow::{anyhow, Context, Result};
use bstr::{io::BufReadExt, ByteSlice};
use clap::Parser;
use std::cmp::Ordering;
use std::collections::{hash_map, HashMap, HashSet};
use std::hash::{BuildHasher, BuildHasherDefault, Hasher};
use std::io::{stdin, stdout, BufRead, Write};
use std::mem;
use std::{default::Default, slice};

mod uniq_iter;

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
    let mut set = HashMap::<Vec<u8>, u64, ahash::RandomState>::default();
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

    if let Some(sort) = sort {
        sort_and_print(delim, sort, &set)
    } else {
        print_out(delim, set.iter().map(|(k, v)| (k.as_slice(), *v)))
    }?;

    std::process::exit(0);
}

type DataAndCount<'a> = (&'a [u8], u64);

/// Sorts the lines by occurence, then prints them
// TODO: this could be done more efficiently by reusing the memory of the HashMap
fn sort_and_print(
    delim: u8,
    sort: Sort,
    set: &HashMap<Vec<u8>, u64, ahash::RandomState>,
) -> Result<()> {
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
        out.write_all(line)?;
        out.write_all(slice::from_ref(&delim))?;
    }

    Ok(())
}

/// Remove duplicates from stdin and print to stdout.
fn uniq_cmd(delim: u8, include_trailing: bool) -> Result<()> {
    // Line processing/output ///////////////////////
    let out = stdout();
    let inp = stdin();
    let hasher = ahash::RandomState::new();
    let mut out = out.lock();
    let mut set = HashSet::<u64, BuildHasherDefault<IdentityHasher>>::default();

    inp.lock().for_byte_record_with_terminator(delim, |line| {
        let tok = trim_end(line, delim);
        if set.insert(hash(&hasher, &tok)) {
            out.write_all(line)?;

            if include_trailing && tok.len() == line.len() {
                out.write_all(&[delim])?;
            }
        }
        Ok(true)
    })?;

    mem::forget(set); // app can now exit, so we don't need to wait for this memory to be freed piecemeal

    Ok(())
}

fn trim_end(record: &[u8], delim: u8) -> &[u8] {
    match record.last_byte() {
        Some(b) if b == delim => &record[..record.len() - 1],
        _ => record,
    }
}

/// Remove duplicates from stdin, using a hash table
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Output the amount of times a line was encountered
    #[clap(short, long)]
    count: bool,

    /// Sort output by the number of occurences, in ascending order
    #[clap(short, long)]
    sort: bool,

    /// Sort output by the number of occurences, in descending order
    #[clap(short = 'S', long)]
    sort_descending: bool,

    /// Which delimiter between elements to use. By default `\n` is used
    ///
    /// Only ascii characters are supported as delimiters.
    /// Use sed to turn your delimiter into zero bytes?
    /// $ echo -n "1λ1λ2λ3" | sed 's@λ@\x00@g' | huniq -0 | sed 's@\x00@λ@g'1λ2λ3λ",
    #[clap(short, long, long = "delimiter", default_value = "\n")]
    delim: char,

    /// Use the \0 character as the record delimiter.
    #[clap(short = '0', long)]
    null: bool,

    /// Prevent adding a delimiter to the last record if missing
    #[clap(short = 't', long = "no-trailing-delimiter")]
    no_trailing_delimiter: bool,
}

fn main() -> Result<()> {
    let Args {
        count,
        sort,
        sort_descending,
        mut delim,
        null,
        no_trailing_delimiter,
        ..
    } = Args::parse();

    if null {
        delim = '\0';
    }
    let delim: u8 = delim.try_into().context("delim is not an ascii char")?;

    let sort = match (sort, sort_descending) {
        (true, true) => return Err(anyhow!("cannot specify both --sort and --sort-descending")),
        (true, false) => Some(Sort::Ascending),
        (false, true) => Some(Sort::Descending),
        (false, false) => None,
    };

    match count || sort.is_some() {
        true => count_cmd(delim, sort),
        false => uniq_cmd(delim, !no_trailing_delimiter),
    }
}
