use std::env::consts::EXE_EXTENSION;
use std::path::Path;

fn main() {
    let new_executable = Path::new(&std::env::current_exe().unwrap())
        .with_file_name("hello")
        .with_extension(EXE_EXTENSION);

    if !new_executable.is_file() {
        eprintln!("hello does not exist, run cargo build --example hello first.");
        std::process::exit(1);
    }

    println!("Next time I run, I am the hello executable");
    self_replace::self_replace(&new_executable).unwrap();

    if std::env::var("FORCE_EXIT").ok().as_deref() == Some("1") {
        std::process::exit(0);
    }
}
