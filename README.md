# include-repo

This rust crate implements a macro which embeds the entire git repository the
code resides in into the built executable.

Why might you want this? Well, the primary use-case I can think of (and in fact
what I built it for) is to provide an
[AGPL](https://www.gnu.org/licenses/agpl-3.0.en.html) compliance endpoint.

Providing the source code for your software is trivial if the source code is
built into the binary.

## Usage

This crate is easy to use. Simply write the following:

```rust
#[macro_use]
extern crate include_repo;

include_repo!(SOURCE_CODE);
// Expands to:
// const SOURCE_CODE: [u8; 999] = [128, 80, ...];

// Do whatever you want with 'SOURCE_CODE'; hint, you may wish to use
// `&SOURCE_CODE[..]` since most fnctions don't take fixed-size arrays of exactly
// the length of your repo's size ;)
```

If you don't wish to include quite every file, that's fine too. For example, if you don't want to include contents in your 'img' and 'third\_party' folders, that can be done like so:

```rust
#[macro_use]
extern crate include_repo;

include_repo!(SOURCE_CODE, ":!/img/", ":!/third_party");
// Any valid pathspec (see
// https://git-scm.com/docs/gitglossary#gitglossary-aiddefpathspecapathspec) may
// be used. Pathspecs *must* be string literals. Any number may be provided to
// the macro.
```

## Assumptions

The following assumptions must be true for this crate to work correctly:

* You use `git` for version control and have a modern version of `git` on your path
* You want your code embedded as a giant const in your binary (not e.g. a static file on disk)
* You're okay transitively depending on [proc-macro-hack](https://github.com/dtolnay/proc-macro-hack)

# License

This code is conveniently available under the AGPL.
