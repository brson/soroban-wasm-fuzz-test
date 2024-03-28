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

---

I have done some initial experimients toward making wasm run under
wasmi fuzzable with `cargo-fuzz` / libfuzzer, and understand better
the basic problems that need to be solved.

I think it is doable, but the effort is significant.

We'll end up doing at least the following:

- modifying `wasmi` to register branches and function entries for every wasm module
- implementing a custom `__sanitizer_symbolize_pc` function to symbolize both
  native and wasm function names
- implementing a `soroban fuzz` command to either wrap or replace `cargo-fuzz`,
  because `cargo-fuzz` will need to be invoked with `--sanitizer=none`.

---

There are three basic components that collaborate to fuzz LLVM-compiled code:

**The instrumented code**. Emitted by LLVM. This does several things:
first it emits setup calls to _both_ `__sanitizer_cov_8bit_counters_init`
and `__sanitizer_cov_pcs_init`. the 8 bit counters indicate visits to PCs
(program counters), and the the PCs describe code locations.
Both these functions must be called with the same number of entries or the fuzzer won't work -
this may be way fitzgen never got the sancov crate to do anything useful.
When collecting fuzzing information, libfuzzer cross-references the PCs
for incremented counters to do things like symbolicate function addresses.
The instrumented code also increments the counters on branches.

**Libfuzzer**. This does probably too many things. Primarily it implements
`__sanitizer_cov_8bit_counters_init` and `__sanitizer_cov_pcs_init` and tracks
the coverage. It occassionally calls `__sanitizer_symbolize_pc` and other sanitizer
functions to symbolicate addresses, etc.
It implements a GUI that prints coverage information to the terminal.
I think it implements the default mutator that chooses the next input bytes.

**Some sanitizer**. The sanitizers all seem to implement common `__sanitizer_*`
functions which are called by libfuzzer. This is why e.g. in #1056 we were
able to work around a bug on macos by mysteriously using thread sanitizer instead
of address sanitizer - they both provide the same common functions.
Of particular interest is `__sanitizer_symbolize_pc` which turns a PC into
a function name for display, a very gnarly system-dependent operation.
On linix at least the fuzzer seems to be able to operate, with degraded capabilities,
with no sanitizer at all (passing `--sanitizer=none` to `cargo-fuzz`) - all
the sanitizer functions are "weak".

The big problem we are going to run into is that these components
are designed with the expectation that PCs live in the address space of the
running program; but with the wasm interepreter we may have many running
programs inside the native running program. The big implication of this is
that the existing sanitizers are not sufficient to symbolicate our PCs;
but also we'll need to come up with a scheme to distinguish between PCs of the
native program and PCs of (multiple) instances of wasm programs.

So we'll probably have to write a new library that implements the sanitizer
functions. Linking to a different sanitizer library requires a more complex
invocation of `cargo-fuzz`, with the `--fsanitizer=none` flag, which is probably
reason enough to bury it in a custom `soroban fuzz` subcommand.

Rust fuzzing is usually done with `libfuzzer-sys` which vendors its own copy
of `libfuzzer`. It may not be strictly necessary to fork libfuzzer if we can come
up with a way of encoding wasm PCs in a way that is compatible with libfuzzer;
but we may also find that we either need to fork it to support wasm PCs,
or want to fork it to e.g. improve the GUI experience.
