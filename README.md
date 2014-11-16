# Rust SIMD Example

Just a tiny example of using SIMD ops in Rust. LLVM is very smart,
so manages to optimize both the SIMD and non-SIMD operations to the
same speed.

## Bench Results

On my machine:

```
running 4 tests
test test::test_add_plain ... ignored
test test::test_add_simd ... ignored
test test::bench_add_plain ... bench:        38 ns/iter (+/- 7)
test test::bench_add_simd  ... bench:        38 ns/iter (+/- 6)

test result: ok. 0 passed; 0 failed; 2 ignored; 2 measured
```

