use std::{fs, io, path::Path};

pub fn process(migrations_dir: &Path) -> io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(migrations_dir)?
        .filter_map(Result::ok)
        .collect();
    entries.sort_by_key(|entry| entry.file_name().to_string_lossy().into_owned());
    for entry in entries {
        let entry = entry;
        let file_path = entry.path();
        println!("{:?}", file_path);
    }
    Ok(())
}
