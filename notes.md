https://github.com/stellar/rs-soroban-sdk/issues/1103

---

Call both
__sanitizer_cov_8bit_counters_init
and
__sanitizer_cov_pcs_init

The number of 8bit counters and pcs must match:
libfuzzer will lookup pcs of counters.

Link to _some_ sanitizer.
Sanitizers implement various __sanitizer_* methods.

Esp. __sanitizer_symbolize_pc

This is done by the sanitizers but in a way
that only makes sense for the native memory space.

Custom fuzzers will need to implement the sanitizer interfaces,
and will need a way to distinguish native PCs
from wasm PCs.

```
cargo +nightly fuzz run fuzz_target_2 --sanitizer=none -- -print_pcs=1
```

