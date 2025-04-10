use std::env;
use std::io;
use std::path::Path;

mod migrations;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!(
            "Usage: {} <migrations_dir_path> <statements_dir_path>",
            args[0]
        );
        std::process::exit(1);
    }

    let migrations_dir_path = &args[1];
    let migrations_dir = Path::new(migrations_dir_path);

    let statements_dir_path = &args[2];
    let statements_dir = Path::new(statements_dir_path);

    let _ = migrations::process(migrations_dir);
    Ok(())
}
