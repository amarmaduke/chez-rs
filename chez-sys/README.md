# chez-sys

Currently only supported 64 bit Linux with curses and x11 disabled.

Note all header functions are implemented (some functional `#define`s are missing).

## Boot files

Boot files for the target OS are constructed and moved to Cargo's `OUT_DIR`.
File specific environment variables are also set:

- `petite.boot` can be accessed through `PETITE_BOOT_PATH`
- `scheme.boot` can be accessed through `SCHEME_BOOT_PATH`

These boot files are needed to start the Scheme interpreter (which is in turn needed to do any interaction at all).
This is not a problem that `chez-sys` will solve.

## Downloading the repository instead of vendoring it

Unfortunately, the ChezScheme repository includes boot files for every supported target in its repository.
This makes perfect sense, because they're needed for bootstrapping, but it causes the cargo package size to be over 10MB.
If you only use _one_ boot file then the package is at around 7MB.

However, creating distinct `chez-arch-sys` crates also feels ridiculous and would cause a total upload of ~70MB to `crates.io` instead of ~30MB for a crate that packages everything.
Thus, instead of trolling the amazing Rust people, its better to bite the bullet and download.

There are two potential fixes:

1. [Some crates are allowed special privileges to be greater than 10MB](https://github.com/rust-lang/crates.io/issues/40#issuecomment-157919165), but it is not clear that a `chez-sys` crate fits the requirements for that. Some time should be spent to see how much traction and use this crate receives before requesting special privileges.

2. It is theoretically possible that a universal boot file could be constructed by compiling to WASM. Then a [WASM interpreter](https://crates.io/crates/wasmer) could be used to bootstrap the process. Upstream would add this boot file to the list of all the others, and `chez-sys` would exclude everything but that boot file from packaging. This has other benefits as allowing Chez Scheme REPLs on the web, so there may be some traction there.
