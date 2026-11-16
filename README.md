# Taqseem

Dead simple tool to split a given instance's group bill between members,
assuming payments were made disproportionately.

It assumes that the total amount paid is equivalent to that of the total bill,
and takes as input a simple CSV file, via STDIN, comprising two headers --
`name` & amount `paid`.

It forgives small payments, from the side of both the recipient, as well as the
ones who must pay. This 'forgiveness amount' is configurable via the `-f` flag.
To disable this 'forgiveness' mechanic, pass the `--strict` flag.

## Usage

```bash
git clone https://github.com/masroof-maindak/taqseem.git
cd taqseem
cargo build
./target/debug/taqseem --help

# Run
./target/debug/taqseem < file.csv | tee result.md

# Install
cargo install --path .
```

## Input Format

```csv
name,paid
alice,100
bob,300
charlie,0
```

## Sample Output

```txt
Total bill: 400
Head count: 3
Bill per head: 133

`bob` paid extra @ 300; they deserve 167
	`alice` (paid 100 so far) has to pay `bob`, 33
	`charlie` (paid 0 so far) has to pay `bob`, 133
```
