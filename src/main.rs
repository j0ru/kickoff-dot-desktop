#![feature(string_remove_matches)]
use freedesktop_entry_parser::{parse_entry, Entry};
use glob::{glob, PatternError};
use std::path::PathBuf;

fn main() {
    let termianal_prefix = "foot ";
    let desktop_files = get_desktop_files().unwrap();
    let mut desktop_entries: Vec<DesktopEntry> = Vec::new();
    for f in desktop_files {
        let entry = parse_entry(f).unwrap();
        if let Ok(entry) = entry.try_into() {
            desktop_entries.push(entry);
        };
    }

    for entry in desktop_entries {
        if entry.terminal {
            println!("{}={}{}", entry.name, termianal_prefix, entry.exec);
        } else {
            println!("{}={}", entry.name, entry.exec);
        }
    }
}

fn get_desktop_files() -> Result<Vec<PathBuf>, PatternError> {
    let desktop_dirs = vec![
        "/usr/share/applications/*.desktop",
        "/usr/local/share/applications/*.desktop",
        "~/.local/share/applications/*.desktop",
    ];

    let mut res = Vec::new();

    for dir in desktop_dirs {
        for entry in glob(dir)?.flatten() {
            res.push(entry)
        }
    }

    Ok(res)
}

enum DesktopError {
    ValuesMissing,
}

impl TryFrom<Entry> for DesktopEntry {
    type Error = DesktopError;

    fn try_from(value: Entry) -> Result<Self, Self::Error> {
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
                    name: name.to_string(),
                    exec,
                    terminal: terminal == "true",
                });
            }
        }
        Err(DesktopError::ValuesMissing)
    }
}

struct DesktopEntry {
    name: String,
    exec: String,
    terminal: bool,
}
