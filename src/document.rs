use std::fmt::{self, Display};

use crate::block::BlockRange;

pub struct Document {
    blocks: Vec<BlockRange>,
}

impl Document {
    pub fn new(ranges: &[(&'static str, u32, u32)]) -> Self {
        let blocks = ranges.iter().copied().map(BlockRange::from).collect();

        Self { blocks }
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "# Useful TUI Characters\n")?;
        writeln!(f, "Escape sequences use Rust `\\u{{XXXX}}` string format.")?;
        writeln!(
            f,
            "Character names are official designations from the [Unicode Character Database](https://www.unicode.org/ucd/).\n"
        )?;

        for block in &self.blocks {
            writeln!(f, "{}", block.toc_entry())?;
        }

        writeln!(f)?;

        for block in &self.blocks {
            write!(f, "{block}\n")?;
        }

        Ok(())
    }
}
