use freedesktop_entry_parser::{parse_entry, Entry};
use std::{fs::read_dir, path::Path};

pub fn get_desktop_entries<P: AsRef<Path>>(
    p: P,
    id_prefix: Option<&str>,
) -> std::io::Result<Vec<DesktopEntry>> {
    let mut res = Vec::new();
    for entry in read_dir(p)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        let filename_raw = entry.file_name();
        let filename = filename_raw.to_str().unwrap();

        // Symlinks are currently not supported
        if filetype.is_file() && entry.path().extension().is_some_and(|ext| ext == "desktop") {
            let desktop_file = parse_entry(entry.path()).unwrap();
            let id = if let Some(prefix) = id_prefix {
                format!("{prefix}-{filename}")
            } else {
                filename.to_string()
            };
            if let Ok(entry) = DesktopEntry::from_entry(desktop_file, id) {
                res.push(entry);
            }
        } else if filetype.is_dir() {
            res.append(&mut get_desktop_entries(entry.path(), Some(filename))?);
        }
    }

    Ok(res)
}

#[derive(Debug)]
enum DesktopError {
    ValuesMissing,
}

impl DesktopEntry {
    fn from_entry(value: Entry, id: String) -> Result<Self, DesktopError> {
        let section = value.section("Desktop Entry");
        if let Some(name) = section.attr("Name") {
            if let Some(exec) = section.attr("Exec") {
                let exec = [
                    "%f", "%F", "%u", "%U", "%d", "%D", "%n", "%N", "%i", "%c", "%k", "%v", "%m",
                ]
                .iter()
                .fold(exec.to_string(), |acc, s| acc.replace(s, ""));

                let terminal = section.attr("Terminal").unwrap_or("false");
                return Ok(DesktopEntry {
                    id,
                    name: name.to_string(),
                    skip: section
                        .attr("NoDisplay")
                        .map(|val| val == "true")
                        .unwrap_or(false),
                    exec,
                    terminal: terminal == "true",
                });
            }
        }
        Err(DesktopError::ValuesMissing)
    }
}

pub struct DesktopEntry {
    pub name: String,
    pub exec: String,
    pub terminal: bool,
    pub skip: bool,
    pub id: String,
}
