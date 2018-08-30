#[macro_use]
extern crate proc_macro_hack;

#[allow(unused_imports)]
#[macro_use]
extern crate include_repo_impl;
#[doc(hidden)]
pub use include_repo_impl::*;

proc_macro_item_decl! {
    /// Create a tarball of the git repo for this project.
    /// This macro must be passed a name which will be used to create a const byte array.
    ///
    /// That is to say, `include_repo!(FOO);` will become `const FOO: [u8; ...] = [...];`
    include_repo! => include_repo_tarball
}
proc_macro_item_decl! {
    /// Create a gzipped tarball of the git repo for this project
    /// This macro must be passed a name which will be used to create a const byte array.
    ///
    /// That is to say, `include_repo_gz!(FOO);` will become `const FOO: [u8; ...] = [...];`
    include_repo_gz! => include_repo_targz
}
