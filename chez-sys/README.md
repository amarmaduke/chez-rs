# chez-sys

Currently only supported 64 bit Linux with curses and x11 disabled.

Note all header functions are implemented (some functional `#define`s are missing).

The (current) build setup is dirty (builds inside the submodule).

## Boot files

Boot files for the target OS are constructed and moved to Cargo's `OUT_DIR`.
File specific environment variables are also set:

- `petite.boot` can be accessed through `PETITE_BOOT_PATH`
- `scheme.boot` can be accessed through `SCHEME_BOOT_PATH`

These boot files are needed to start the Scheme interpreter (which is in turn needed to do any interaction at all).
This is not a problem that `chez-sys` will solve.
