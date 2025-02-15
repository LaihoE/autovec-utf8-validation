# Autovectorized utf8 validator
Godbolt: https://rust.godbolt.org/z/qrabTh3d3

Autovectorized utf-8 validator in 100% safe rust. This algorithm is meant for non-ascii text. Ascii fast paths can be built on top of this.

### Run bench
```
RUSTFLAGS='-C target-cpu=native' cargo bench
```
**simdutf8 uses runtime feature detection**. To get a fair comparison run the above.

### Check correctness
```
cargo run
```

### Results on my machine (Ryzen 5900x)
```
test bench_chinese_std               ... bench:       4,755.05 ns/iter (+/- 74.27)
test bench_simd_chinese_autovec      ... bench:         995.15 ns/iter (+/- 6.30)
test bench_simd_chinese_autovec_max3 ... bench:         668.32 ns/iter (+/- 4.86)
test bench_simd_chinese_simdutf8     ... bench:         420.27 ns/iter (+/- 2.68)
```
The comparison is still not 100% fair as simdutf8 has an ascii fast path that cannot be used in this benchmark as all chunks contain non-ascii chars.


autovec_max3 is a version that only **validates** utf8 of length 3 and **recognizes** utf8 of length 4. See below for more info.


### Why would you want to validate utf of len 3 and only recognize len 4?

4-long utf8 is very rare. Mostly emojis, mathematical symbols and very rare chinese/japanse characters. Code can be adjusted to check in chunks and fallback to slower algorithms if a chunks contains a 4-long utf8. 