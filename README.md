# Rust SIMD Example

Just a tiny example of using SIMD ops in Rust to speed up vector addition
almost 20x.

## Usage

```bash
make lib && make test && make bench
```

## Bench Results

On my machine:

```
running 4 tests
test test::test_add_plain ... ignored
test test::test_add_simd ... ignored
test test::bench_add_plain ... bench:       806 ns/iter (+/- 191)
test test::bench_add_simd  ... bench:        52 ns/iter (+/- 14)

test result: ok. 0 passed; 0 failed; 2 ignored; 2 measured
```

