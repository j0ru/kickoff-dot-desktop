# kickoff-dot-desktop

Smol program to read in relevant desktop files and print them in a kickoff compatible format

## Installation
```bash
git clone https://github.com/j0ru/kickoff-dot-desktop.git
cd kickoff-dot-desktop
cargo install --path .
```

## Usage

```bash
kickoff-dot-desktop | kickoff --from-stdin
```

A custom terminal can be set with the TERMINAL env variable. If not found, a list of common terminals will be tested.