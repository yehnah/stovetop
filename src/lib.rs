use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::str;

use toml::Value;

// ─────────────────────────────────────────────────────────────────────────────

pub fn read_directory(path: &str) -> std::io::Result<Vec<PathBuf>> {
    let mut store: Vec<PathBuf> = vec![];
    for entry in fs::read_dir(path.to_string())? {
        let dir = entry?.path();
        store.push(dir);
    }
    Ok(store)
}

// ─────────────────────────────────────────────────────────────────────────────

pub fn read_file(path: &str) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut string_buffer = String::new();
    f.read_to_string(&mut string_buffer)?;
    Ok(string_buffer)
}

// ─────────────────────────────────────────────────────────────────────────────

pub fn generate(
    path: &str,
    template_path: &str,
    output_path: &str,
    recursive_path: Option<&str>,
) -> io::Result<()> {
    let main_path = Path::new(path);
    let output_base = Path::new(output_path);
    if !main_path.is_dir() {
        panic!("Path {} is not a directory", main_path.to_str().unwrap())
    }
    if !output_base.is_dir() {
        fs::create_dir(&output_path).unwrap();
    }
    // Read list of files
    let files = read_directory(path).unwrap();

    // Gather template toml data
    let template_data = read_file(template_path)?;
    let template_values = template_data.parse::<Value>().unwrap();

    // - CONVERT TOML TO HASHMAP -----------------------------------------------
    // Read template data into a hashmap to get access to Table keys as an iter.

    let replacers = &template_values.try_into::<HashMap<String, String>>()?;

    // - ENTRY LOOP ------------------------------------------------------------
    // For each entry replace the templates and write the file.

    for entry in &files {
        // - CASE: IS DIRECTORY --------------------------------------------
        // Create the directory to match the template.
        if entry.is_dir() {
            let pth = format!(
                "{output}/{name}",
                output = output_path,
                name = entry.file_name().unwrap().to_str().unwrap()
            );

            fs::create_dir(&pth).unwrap();
            generate(entry.to_str().unwrap(), template_path, output_path, Some(path)).unwrap();
        }
        // - CASE: IS FILE -------------------------------------------------
        // Replace the template values in each file and write to output
        else {
            if entry.as_os_str() != OsStr::new(template_path) {
                // Ensure we're not copying the template data
                let mut file_string = read_file(entry.to_str().unwrap()).unwrap();

                // Replace template value in the output string
                for replacer in replacers.keys() {
                    let replaced = str::replace(
                        &file_string,
                        &format!("{}{}{}", "{{", replacer, "}}"),
                        replacers.get(replacer).unwrap(),
                    );

                    file_string = replaced;
                }

                // - WRITE FILE ------------------------------------------------

                let data = &file_string.into_bytes();
                let mut pos = 0;

                // Replace input path with output path
                let pth = str::replace(entry.to_str().unwrap(), recursive_path.unwrap_or(path), output_path);
                let new_file_path = PathBuf::from(pth);
                let mut buffer = File::create(new_file_path)?;
                while pos < data.len() {
                    let bytes_written = buffer.write(&data[pos..])?;
                    pos += bytes_written;
                }
            }
        }
    }
    Ok(())
}

// ═════════════════════════════════════════════════════════════════════════════
// - TESTS ---------------------------------------------------------------------
// ═════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn has_all_files() {
        let val = read_directory("./example/template").unwrap();
        assert_eq!(val.len(), 2);
    }

    #[test]
    fn reads_directory() {
        let val = read_directory("./example/template").unwrap();
        assert_eq!(val[0].to_str().unwrap(), "./example/template/template.txt");
    }

    #[test]
    fn reads_file() {
        let val = read_file("./example/template/template.txt").unwrap();
        assert_eq!(val, "Hello {{title}} from {{place}}");
    }
}
