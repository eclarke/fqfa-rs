extern crate bio;

use std::io::{self, Write};

use bio::io::{fasta, fastq};

fn main() {
    let reader = fastq::Reader::new(io::stdin());
    let mut writer = fasta::Writer::new(io::stdout());
    for (i, record) in reader.records().enumerate() {
        match record {
            Ok(r) => {
                writer.write(
                    r.id().unwrap_or(&format!("{}", i)),
                    r.desc(),
                    r.seq(),
                ).ok().expect("Error writing to stdout!")
            },
            Err(e) => {
                io::stderr().write(&format!("{}", e).as_bytes()).ok();
            }
        }
    }
}
