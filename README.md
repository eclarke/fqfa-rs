# fqfa

A fast pipe-based converter between FASTQ and FASTA.

## Installation

Assuming Rust and Cargo are installed:

```sh
git clone https://github.com/eclarke/fqfa
cd fqfa
cargo install
```

## Usage

`fqfa` accepts FASTQ records from stdin and outputs FASTA records to stdout. That's all.

```sh
fqfa < foo.fq > foo.fa
```
