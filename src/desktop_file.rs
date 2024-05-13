use freedesktop_entry_parser::{parse_entry, Entry};
use std::{fs::read_dir, path::Path};
use thiserror::Error;

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
            if let Ok(entry) = DesktopEntry::from_entry(&desktop_file, id) {
                res.push(entry);
            }
        } else if filetype.is_dir() {
            res.append(&mut get_desktop_entries(entry.path(), Some(filename))?);
        }
    }

    Ok(res)
}

#[derive(Debug, Error)]
enum Error {
    #[error("The desktop file is missing the {} key", self)]
    ValuesMissing(&'static str),
    #[error("The desktop file is not an applicatio")]
    NotAnApplication,
}

impl DesktopEntry {
    fn from_entry(value: &Entry, id: String) -> Result<Self, Error> {
        let section = value.section("Desktop Entry");
        if section.attr("Type") != Some("Application") {
            return Err(Error::NotAnApplication);
        }
        if let Some(name) = section.attr("Name") {
            if let Some(exec) = section.attr("Exec") {
                let exec = [
                    "%f", "%F", "%u", "%U", "%d", "%D", "%n", "%N", "%i", "%c", "%k", "%v", "%m",
                ]
                .iter()
                .fold(exec.to_string(), |acc, s| acc.replace(s, ""));

                let terminal: bool = section
                    .attr("Terminal")
                    .is_some_and(|v| v.parse() == Ok(true));
                Ok(DesktopEntry {
                    id,
                    name: name.to_string(),
                    skip: section.attr("NoDisplay").is_some_and(|val| val == "true"),
                    exec,
                    terminal,
                })
            } else {
                Err(Error::ValuesMissing("Exec"))
            }
        } else {
            Err(Error::ValuesMissing("Name"))
        }
    }
}

pub struct DesktopEntry {
    pub name: String,
    pub exec: String,
    pub terminal: bool,
    pub skip: bool,
    pub id: String,
}
