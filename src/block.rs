use std::fmt::{self, Display};

use unicode_names2::name;
use unicode_width::UnicodeWidthChar;

struct CharEntry {
    ch: char,
    escape: String,
    name: String,
    display_width: usize,
}

impl From<char> for CharEntry {
    fn from(ch: char) -> Self {
        let escape = format!("`\\u{{{:04X}}}`", ch as u32);
        let name = name(ch).map(|n| n.to_string()).unwrap_or_default();
        let display_width = ch.width().unwrap_or(1);

        Self {
            escape,
            name,
            display_width,
            ch,
        }
    }
}

pub struct BlockRange {
    heading: &'static str,
    entries: Vec<CharEntry>,
    max_escape: usize,
    max_name: usize,
}

impl From<(&'static str, u32, u32)> for BlockRange {
    fn from((heading, start, end): (&'static str, u32, u32)) -> Self {
        let entries: Vec<CharEntry> = (start..=end)
            .filter_map(char::from_u32)
            .map(CharEntry::from)
            .collect();

        let max_escape = entries
            .iter()
            .map(|e| e.escape.len())
            .max()
            .unwrap_or(0)
            .max("Escape".len());

        let max_name = entries
            .iter()
            .map(|e| e.name.len())
            .max()
            .unwrap_or(0)
            .max("Name".len());

        Self {
            heading,
            entries,
            max_escape,
            max_name,
        }
    }
}

impl BlockRange {
    fn anchor(&self) -> String {
        format!("{} ({})", self.heading, self.entries.len())
            .to_lowercase()
            .replace(' ', "-")
            .replace(|c: char| !c.is_ascii_alphanumeric() && c != '-', "")
    }

    pub fn toc_entry(&self) -> String {
        format!(
            "- [{}](#{}) ({})",
            self.heading,
            self.anchor(),
            self.entries.len(),
        )
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
