use std::io::Write;

use libflate::gzip::Encoder;
use proc_macro::TokenStream;
use syn::parse_macro_input;

fn repo_tarball(filter: &[String]) -> Vec<u8> {
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

struct Filters {
    filters: Vec<String>
}

impl syn::parse::Parse for Filters {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let mut filters = Vec::new();

        while input.peek(syn::Lit) {
            let lit: syn::Lit = input.parse()?;
            let s = match lit {
                syn::Lit::Str(s) => s.value(),
                syn::Lit::ByteStr(s) => String::from_utf8(s.value()).unwrap(),
                _ => panic!("[include-repo]: error parsing filter argument. Must be a string, got {:?}", lit),
            };
            filters.push(s);
        }
        Ok(Filters{filters})
    }
}

#[proc_macro]
/// Create a tarball of the git repo for this project.
///
/// That is to say, `const FOO = include_repo!();` will become `const FOO: [u8; ...] = [...];`
pub fn include_repo(args: TokenStream) -> TokenStream {
    let f: Filters = parse_macro_input!(args as Filters);
    let tar = repo_tarball(&f.filters);
    format!("&[{}]", tar.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(", ")).parse().unwrap()
}

#[proc_macro]
/// Create a gzipped tarball of the git repo for this project
///
/// That is to say, `const FOO: &[u8] = include_repo_gz!();` will become `const FOO: &[u8] = [<gzipped bytes>];`
pub fn include_repo_gz(args: TokenStream) -> TokenStream {
    let f: Filters = parse_macro_input!(args as Filters);
    let tar = repo_tarball(&f.filters);

    let mut encoder = Encoder::new(Vec::new()).unwrap();
    encoder.write_all(&tar).unwrap();
    let targz = encoder.finish().into_result().unwrap();

    format!("&[{}];", targz.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(", ")).parse().unwrap()
}
