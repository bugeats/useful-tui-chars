use std::fmt::{self, Display};

use unicode_names2::name;
use unicode_width::UnicodeWidthChar;

const RANGES: &[(&str, u32, u32)] = &[
    ("Greek and Coptic", 0x0370, 0x03FF),
    ("Letterlike Symbols", 0x2100, 0x214F),
    ("Arrows", 0x2190, 0x21FF),
    ("Mathematical Operators", 0x2200, 0x22FF),
    ("Miscellaneous Technical", 0x2300, 0x23FF),
    ("Enclosed Alphanumerics", 0x2460, 0x24FF),
    ("Box Drawing", 0x2500, 0x257F),
    ("Block Elements", 0x2580, 0x259F),
    ("Geometric Shapes", 0x25A0, 0x25FF),
    ("Miscellaneous Symbols", 0x2600, 0x26FF),
    ("Dingbats", 0x2700, 0x27BF),
    ("Braille Patterns", 0x2800, 0x28FF),
    ("Miscellaneous Symbols and Arrows", 0x2B00, 0x2BFF),
    ("Symbols for Legacy Computing", 0x1FB00, 0x1FBFF),
];

struct CharEntry {
    ch: char,
    escape: String,
    name: String,
    display_width: usize,
}

impl CharEntry {
    fn new(ch: char) -> Self {
        Self {
            escape: format!("`\\u{{{:04X}}}`", ch as u32),
            name: name(ch).map(|n| n.to_string()).unwrap_or_default(),
            display_width: ch.width().unwrap_or(1),
            ch,
        }
    }
}

struct BlockRange {
    heading: &'static str,
    entries: Vec<CharEntry>,
    max_escape: usize,
    max_name: usize,
}

impl BlockRange {
    fn anchor(&self) -> String {
        format!("{} ({})", self.heading, self.entries.len())
            .to_lowercase()
            .replace(' ', "-")
            .replace(|c: char| !c.is_ascii_alphanumeric() && c != '-', "")
    }

    fn toc_entry(&self) -> String {
        format!(
            "- [{}](#{}) ({})",
            self.heading,
            self.anchor(),
            self.entries.len(),
        )
    }

    fn new(heading: &'static str, start: u32, end: u32) -> Self {
        let entries: Vec<CharEntry> = (start..=end)
            .filter_map(char::from_u32)
            .map(CharEntry::new)
            .collect();

        let max_escape = entries.iter().map(|e| e.escape.len()).max().unwrap_or(0);
        let max_name = entries.iter().map(|e| e.name.len()).max().unwrap_or(0);

        Self {
            heading,
            entries,
            max_escape: max_escape.max("Escape".len()),
            max_name: max_name.max("Name".len()),
        }
    }
}

impl Display for BlockRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let esc = self.max_escape;
        let name = self.max_name;
        let count = self.entries.len();

        writeln!(f, "## {} ({count})\n", self.heading)?;
        writeln!(f, "| Char | {:<esc$} | {:<name$} |", "Escape", "Name")?;
        writeln!(
            f,
            "|------|{:-<w_esc$}|{:-<w_name$}|",
            "",
            "",
            w_esc = esc + 2,
            w_name = name + 2
        )?;

        for entry in &self.entries {
            let pad = 4 - entry.display_width;

            writeln!(
                f,
                "| {}{:<pad$} | {:<esc$} | {:<name$} |",
                entry.ch, "", entry.escape, entry.name,
            )?;
        }

        Ok(())
    }
}

fn main() {
    let blocks: Vec<BlockRange> = RANGES
        .iter()
        .map(|&(heading, start, end)| BlockRange::new(heading, start, end))
        .collect();

    println!("# Useful TUI Characters\n");
    println!("Escape sequences use Rust `\\u{{XXXX}}` string format.");
    println!("Character names are official designations from the [Unicode Character Database](https://www.unicode.org/ucd/).\n");

    for block in &blocks {
        println!("{}", block.toc_entry());
    }

    println!();

    for block in &blocks {
        println!("{block}");
    }
}
