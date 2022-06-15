# include-repo

The `include_repo` rust crate provides a macro which embeds all files in the
project's git repository into the final executable. It does not embed git
history nor metadata, but rather includes a tarball much like
[`git archive`](https://git-scm.com/docs/git-archive) would produce.

Why might you want this? Well, the primary use-case I can think of (and in fact
what I built it for) is to provide an
[AGPL](https://www.gnu.org/licenses/agpl-3.0.en.html) compliance endpoint.

Providing the source code for your software is trivial if the source code is
built into the binary.

## Usage

This crate is easy to use. Simply include the following in your code somewhere:

```rust
use include_repo::include_repo;

const SOURCE_CODE: &[u8] = include_repo!();
// Expands to:
// const SOURCE_CODE: &[u8] = [128, 80, ...];
// The bag of bytes is a tarball, so serve it with a .tar extension please!
```

If you don't wish to include quite every file, that's also possible. For
example, if you don't want to include contents in your 'img' and 'third\_party'
folders, that can be done like so:

```rust
use include_repo::include_repo;

const SOURCE_CODE: &[u8] = include_repo!(".", ":!/img/", ":!/third_party");
// Any valid pathspec (see
// https://git-scm.com/docs/gitglossary#gitglossary-aiddefpathspecapathspec) may
// be used. Pathspecs *must* be string literals. Any number may be provided to
// the macro.
// The "." portion is optional on newer versions of git, but for backwards
// compatibility it's best to add it if all other pathspecs are exclusions.
```

If you want the tarball to be gzipped, use the `include_repo_gz!` macro instead. If you don't already have gzip decompression included in your binary, you may find it easier to shell out to `tar -xzv -f -` when the source is requested, or provide a `.tar.gz` file to users.

## Assumptions

The following assumptions must be true for this crate to work correctly:

* You use `git` for version control and have a modern version of `git` in your `PATH`
* You want to provide your source code as a tarball (optionally gzipped), not a zip or something
* You want your code embedded as a giant const in your binary (not e.g. a static file on disk)
* You don't mind a proc macro running 'git' as part of your build

# License

This code is conveniently available under the AGPL.
