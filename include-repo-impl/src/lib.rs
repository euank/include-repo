#[macro_use]
extern crate proc_macro_hack;
extern crate libflate;
#[macro_use]
extern crate syn;

use libflate::gzip::Encoder;
use std::io::Write;
use syn::punctuated::Punctuated;
use syn::synom::Parser;
use syn::{Expr, Lit};

fn repo_tarball(filter: &Vec<String>) -> Vec<u8> {
    // in an ideal world, this would probably be implemented with git2-rs or such.
    // In my world, it's a lot easier to just do this, and this is a bit of a hack in the first
    // place.
    let toplevel = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        .expect("could not get top level directory");
    let toplevel_dir = String::from_utf8(toplevel.stdout).unwrap();
    let toplevel_dir = toplevel_dir.trim();
    let mut archive = std::process::Command::new("git");
    archive
        .current_dir(toplevel_dir)
        .arg("archive")
        .arg("HEAD")
        .arg("--");
    for f in filter {
        archive.arg(f);
    }
    let output = archive
        .output()
        .expect("could not run 'git archive HEAD' for repo_tarball");

    if !output.status.success() {
        panic!("[include-repo]: error running git-archive: Exit {}: {}", output.status, String::from_utf8_lossy(&output.stderr));
    }
    output.stdout
}

fn parse_input(input: &str) -> (String, Vec<String>) {
    let parts = input.splitn(2, ",").collect::<Vec<_>>();
    let const_name = parts[0];
    if parts.len() == 1 {
        // perfectly fine to not have any git filters
        return (const_name.to_string(), Vec::new());
    }
    let git_filter = parts[1];

    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let args = parser.parse_str(git_filter).unwrap();
    let filters: Vec<String> = args
        .iter()
        .map(|item| match item {
            Expr::Lit(lit) => match lit.lit {
                Lit::Str(ref s) => return s.value(),
                _ => {
                    panic!("[include-repo] git filters must be string literals");
                }
            },
            _ => {
                panic!("[include-repo] git filters must be string literals");
            }
        }).collect();

    return (const_name.to_string(), filters);
}

proc_macro_item_impl! {
    pub fn include_repo_tarball(input: &str) -> String {
        let (const_name, filters) = parse_input(input);
        let tar = repo_tarball(&filters);
        format!("const {}: [u8; {}] = [{}];", const_name, tar.len(), tar.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(", "))
    }

    pub fn include_repo_targz(input: &str) -> String {
        let (const_name, filters) = parse_input(input);
        let tar = repo_tarball(&filters);

        let mut encoder = Encoder::new(Vec::new()).unwrap();
        encoder.write_all(&tar).unwrap();
        let targz = encoder.finish().into_result().unwrap();

        format!("const {}: [u8; {}] = [{}];", const_name, targz.len(), targz.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(", "))
    }
}
