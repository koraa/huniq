extern crate clap;

use std::collections::{HashSet, HashMap, hash_map, hash_map::DefaultHasher};
use std::hash::{Hasher};
use std::io::{stdin, BufRead, BufReader, stdout, Write, BufWriter};
use std::slice;
use clap::{Arg, App};
use failure::Error;

type Result<T> = std::result::Result<T, Error>;

fn count_cmd(delim: u8) -> Result<()> {
    let mut set = HashMap::<Vec<u8>, u64>::new();
    for line in BufReader::new(stdin()).split(delim) {
        match set.entry(line?) {
            hash_map::Entry::Occupied(mut e) => { *e.get_mut() += 1; },
            hash_map::Entry::Vacant(e)   => { e.insert(1); }
        }
    }

    let mut out = BufWriter::new(stdout());
    for (line, count) in set.iter() {
        write!(out, "{} ", count)?;
        out.write(&line)?;
        out.write(slice::from_ref(&delim))?;
    }

    Ok(())
}

fn hash<T: std::hash::Hash>(v: &T) -> u64 {
    let mut s = DefaultHasher::new();
    v.hash(&mut s);
    s.finish()
}

fn uniq_cmd(delim: u8) -> Result<()> {
    let mut out = BufWriter::new(stdout());
    let mut set = HashSet::<u64>::new();

    for line in BufReader::new(stdin()).split(delim) {
        let line = line?;
        if set.insert(hash(&line)) {
            out.write(&line)?;
            out.write(slice::from_ref(&delim))?;
        }
    }

    Ok(())
}


fn try_main() -> Result<()> {
    let mut argspec = App::new("huniq")
        .version("2.0.2")
        .about("Remove duplicates from stdin, using a hash table")
        .author("Karolin Varner <karo@cupdev.net)")
        .arg(Arg::with_name("count")
            .help("Output the amount of times a line was encountered")
            .long("count")
            .short("c"))
        .arg(Arg::with_name("delimiter")
            .help("Which delimiter between elements to use. By default `\n` is used")
            .long("delimiter")
            .long("delim")
            .short("d")
            .takes_value(true)
            .default_value("\n")
            .validator(|v| match v.len() {
                1 => Ok(()),
                _ => Err(String::from("\
Only ascii characters are supported as delimiters. \
Use sed to turn your delimiter into zero bytes?

    $ echo -n \"1λ1λ2λ3\" | sed 's@λ@\x00@g' | huniq -0 | sed 's@\x00@λ@g'
    1λ2λ3λ"
                ))
            }))
        .arg(Arg::with_name("null")
            .help("Use the \\0 character as the record delimiter.")
            .long("null")
            .short("0")
            .conflicts_with("delimiter"));

    let args = argspec.get_matches_from_safe_borrow(&mut std::env::args_os())?;

    let delim = match args.is_present("null") {
            true  => b'\0',
            false => args.value_of("delimiter").unwrap().as_bytes()[0]
    };

    match args.is_present("count") {
        true  => count_cmd(delim),
        false => uniq_cmd(delim)
    }
}

fn main() {
    if let Err(er) = try_main() {
        println!("{}", er);
    }
}
