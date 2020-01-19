use std::collections::{HashSet, HashMap, hash_map, hash_map::DefaultHasher};
use std::hash::{Hasher, BuildHasher};
use std::io::{stdin, Read, BufRead, BufReader, stdout, Write, BufWriter, ErrorKind};
use std::slice;
use sysconf::page::pagesize;
use anyhow::Result;
use clap::{Arg, App};
use fxhash::FxBuildHasher;

struct IdentityHasher {
    off: u8,
    buf: [u8; 8],
}

impl Hasher for IdentityHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.off += (&mut self.buf[self.off as usize..])
            .write(bytes).unwrap_or(0) as u8;
    }

    fn finish(&self) -> u64 {
        u64::from_ne_bytes(self.buf)
    }
}

#[derive(Default)]
struct BuildIdentityHasher {}

impl BuildHasher for BuildIdentityHasher {
    type Hasher = IdentityHasher;

    fn build_hasher(&self) -> Self::Hasher {
        IdentityHasher { off: 0, buf: [0; 8] }
    }
}

fn calc_hash<T: BuildHasher, U: std::hash::Hash + ?Sized>(build: &T, v: &U) -> u64 {
    let mut s = build.build_hasher();
    v.hash(&mut s);
    s.finish()
}

fn count_cmd(delim: u8) -> Result<()> {
    let mut set = HashMap::<Vec<u8>, u64>::new();
    for line in BufReader::new(stdin().lock()).split(delim) {
        match set.entry(line?) {
            hash_map::Entry::Occupied(mut e) => { *e.get_mut() += 1; },
            hash_map::Entry::Vacant(e)   => { e.insert(1); }
        }
    }

    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for (line, count) in set.iter() {
        write!(out, "{} ", count)?;
        out.write(&line)?;
        out.write(slice::from_ref(&delim))?;
    }

    Ok(())
}

// Remove duplicates from stdin and print to stdout.
// Optimizations used:
// * Use lock() on stdin/stout
// * Use manual buffering to avoid the extra copy incured with
fn uniq_cmd(delim: u8) -> Result<()> {
    // Line processing/output ///////////////////////

    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    let hasher = FxBuildHasher::default();
    let mut set = HashSet::<u64, BuildIdentityHasher>::default();

    // Handler: We managed to find a line! Process the line
    // The line *always* includes the delimiter at the end
    let mut found_line = |line: &[u8]| -> Result<()> {
        let tok: &[u8] = &line[..line.len()-1];
        if set.insert(calc_hash(&hasher, &tok)) {
            out.write(&line)?;
        }
        Ok(())
    };

    // Line Spitting/input //////////////////////////

    let inp = stdin();
    let mut inp = inp.lock();

    // Our manually managed stdin buffer; using BufReader would not
    // allow us to control the buffer size which we need to support
    // very long lines in the primary buffer.
    let mut buf = Vec::<u8>::new();
    buf.resize(pagesize() * 2, 0);

    // Amount of data actually in the buffer (manually managed so we
    // can avoid reinitializing all the data when we resize)
    let mut used: usize = 0;

    loop {
        match inp.read(&mut buf[used..] /* unused space */) {
            Err(ref e) if e.kind() == ErrorKind::Interrupted =>
                // EINTR – thrown when a process receives a signal…
                continue,
            Err(e) =>
                return Err(anyhow::Error::from(e)),
            Ok(0) => {
                // EOF; we potentially need to process the last word here
                // if the input is missing a newline (or rather delim) at
                // the end of it's input
                if used != 0 {
                    // Grow the buffer if this is necessary to insert the delimiter
                    if used == buf.len() {
                        buf.push(delim);
                    } else {
                        buf[used] = delim;
                    }
                    used += 1;

                    found_line(&buf[..used])?;
                }
                break;
            }, Ok(len) =>
                // Register the data that became available
                used += len
        };

        // Scan the buffer for lines
        let mut line_start: usize = 0;
        for (off, chr) in (&buf[..used]).iter().enumerate() {
            if *chr == delim {
                found_line(&buf[line_start..off+1])?;
                line_start = off + 1;
            }
        }

        // Move the current line fragment to the start of the buffer
        // so we can fill the rest
        buf.copy_within(line_start.., 0);
        used = used - line_start; // Length of the rest

        // Grow the buffer if necessary, letting Vec decide what growth
        // factor to use
        if used == buf.len() {
            buf.resize(buf.len() + 1, 0);
            buf.resize(buf.capacity(), 0);
        }
    }

    Ok(())
}


fn try_main() -> Result<()> {
    let mut argspec = App::new("huniq")
        .version("2.0.3")
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
