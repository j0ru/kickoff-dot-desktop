use freedesktop_entry_parser::{parse_entry, Entry};
use glob::{glob, PatternError};
use std::path::PathBuf;

fn main() {
    let termianal_prefix = find_terminal();
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
            println!("{}={} {}", entry.name, termianal_prefix, entry.exec);
        } else {
            println!("{}={}", entry.name, entry.exec);
        }
    }
}

/// reimplementation of i3-sensible-terminal
/// tests for $TERMINAL or returns the first existing terminal
/// from a list of common terminals
fn find_terminal() -> String {
    if let Ok(term) = std::env::var("TERMINAL") {
        return term;
    };

    for term in [
        "x-terminal-emulator",
        "mate-terminal",
        "gnome-terminal",
        "terminator",
        "xfce4-terminal",
        "foot",
        "urxvt",
        "rxvt",
        "termit",
        "Eterm",
        "aterm",
        "uxterm",
        "xterm",
        "roxterm",
        "termite",
        "lxterminal",
        "terminology",
        "st",
        "qterminal",
        "lilyterm",
        "tilix",
        "terminix",
        "konsole",
        "kitty",
        "guake",
        "tilda",
        "alacritty",
        "hyper",
        "wezterm",
    ] {
        if let Ok(path) = which::which(term) {
            return path.to_str().unwrap().to_string();
        }
    }
    panic!("no matching terminal found")
}

fn get_desktop_files() -> Result<Vec<PathBuf>, PatternError> {
    let default_dirs = vec![
        "/usr/share/applications/*.desktop".to_string(),
        "/usr/local/share/applications/*.desktop".to_string(),
        "~/.local/share/applications/*.desktop".to_string(),
    ];
    let desktop_dirs: Vec<String> = if let Ok(paths) = std::env::var("XDG_DATA_DIRS") {
        if !paths.is_empty() {
            let paths: Vec<String> = std::env::split_paths(&paths)
                .into_iter()
                .map(|mut path| {
                    path.push("applications/**/*.desktop");
                    path.to_str().unwrap().to_owned()
                })
                .collect();

            paths
        } else {
            default_dirs
        }
    } else {
        default_dirs
    };

    let mut res = Vec::new();

    for dir in desktop_dirs {
        for entry in glob(&dir)?.flatten() {
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
