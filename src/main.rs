use std::collections::HashSet;

use desktop_file::{get_desktop_entries, DesktopEntry};
use lazy_static::lazy_static;

mod desktop_file;

lazy_static! {
    static ref TERMINAL: String = find_terminal();
}

fn main() {
    let home_dir = std::env::var("HOME").expect("HOME not set");
    let mut dirs = vec![
        "/usr/share/applications/".to_string(),
        "/usr/local/share/applications/".to_string(),
        format!("{}/.local/share/applications/", home_dir),
    ];
    if let Ok(paths) = std::env::var("XDG_DATA_DIRS") {
        if !paths.is_empty() {
            let mut paths: Vec<String> = std::env::split_paths(&paths)
                .map(|mut path| {
                    path.push("applications/");
                    path.to_str().unwrap().to_owned()
                })
                .collect();

            dirs.append(&mut paths);
        }
    }

    let mut known_ids = HashSet::new();
    let mut desktop_entries: Vec<DesktopEntry> = Vec::new();

    for dir in dirs {
        let entries = match get_desktop_entries(&dir, None) {
            Err(e) => {
                eprintln!("problem when fetching from {dir}: {e}");
                continue;
            }
            Ok(e) => e,
        };
        for entry in entries {
            if known_ids.insert(entry.id.clone()) {
                desktop_entries.push(entry);
            }
        }
    }

    for entry in desktop_entries {
        if !entry.skip {
            if entry.terminal {
                println!("{}={} {}", entry.name, *TERMINAL, entry.exec)
            } else {
                println!("{}={}", entry.name, entry.exec)
            }
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
