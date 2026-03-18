mod block;
mod document;

use document::Document;

const RANGES: &[(&str, u32, u32)] = &[
    ("Box Drawing", 0x2500, 0x257F),
    ("Block Elements", 0x2580, 0x259F),
    ("Arrows", 0x2190, 0x21FF),
    ("Geometric Shapes", 0x25A0, 0x25FF),
    ("Mathematical Operators", 0x2200, 0x22FF),
    ("Greek and Coptic", 0x0370, 0x03FF),
    ("Letterlike Symbols", 0x2100, 0x214F),
    ("Braille Patterns", 0x2800, 0x28FF),
    ("Miscellaneous Technical", 0x2300, 0x23FF),
    ("Dingbats", 0x2700, 0x27BF),
    ("Enclosed Alphanumerics", 0x2460, 0x24FF),
    ("Miscellaneous Symbols", 0x2600, 0x26FF),
    ("Miscellaneous Symbols and Arrows", 0x2B00, 0x2BFF),
    ("Symbols for Legacy Computing", 0x1FB00, 0x1FBFF),
];

fn main() {
    print!("{}", Document::new(RANGES));
}
