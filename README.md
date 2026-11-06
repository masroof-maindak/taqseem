# Taqseem

Dead simple tool to split a given instance's group bill between members,
assuming payments were made disproportionately.

It assumes that the total amount paid is equivalent to that of the total bill,
and takes as input a simple CSV file, via STDIN, comprising two headers --
`name` & amount `paid`.

## Usage

```bash
git clone https://github.com/masroof-maindak/taqseem.git
cd taqseem
cargo build
./target/debug/taqseem < file.csv | tee result.md
```

## Input Format

```csv
name,paid
alice,100
bob,300
charlie,0
```
