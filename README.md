# wc-rs
A wc clone written in rust (currently only works for text files)

## Usage

```
Print different counts for FILE, when no file is specified data is read from stdin

Usage: wc-rs [OPTIONS] [FILE]

Arguments:
  [FILE]  file to be read, when no file is specified data is read from stdin

Options:
  -b, --bytes            print the byte counts
  -c, --chars            print the character counts
  -l, --lines            print the newline counts
  -m, --max-line-length  print the maximum display width
  -w, --words            print the word counts
  -h, --help             Print help
  -V, --version          Print version
```
