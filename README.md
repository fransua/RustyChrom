# Simple restriction-enzyme splitter written in Rust

## build with cargo

### Create repo

```
cargo new RustyChrom --bin
```

### Compile

with Rust:

```
rustc src/bin/bin.rs
```
This will create an executable `bin`

with Cargo:
```
cargo build
```
This will create an executable `RustyChrom` in the `target/debug` folder


## run

Rust implementation 1:
```
$ time ./bin_while test/sample.fq GATC
0,54s user 0,51s system 99% cpu 1,056 total
```

Rust implementation 2:
```
$ time ./bin_while test/sample.fq GATC
0,52s user 0,74s system 99% cpu 1,260 total
```

Python implementation:
```
time python src/bin/split_by_restriction_site.py test/sample.fq GATC
0,16s user 0,02s system 96% cpu 0,184 total
```

... shit