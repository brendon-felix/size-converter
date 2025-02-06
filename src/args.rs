use crate::convert::{Size, Value, Unit, Base};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub bits: bool,
    /// Size in bytes/bits to convert
    pub size: String,
    /// Convert to new unit/base
    pub to: Option<String>,
}

pub fn parse_unit_base(s: &str) -> (Unit, Base) {
    let unit = match s {
        s if s.starts_with("Ki") => Unit::Kibi,
        s if s.starts_with("Mi") => Unit::Mibi,
        s if s.starts_with("Gi") => Unit::Gibi,
        s if s.starts_with("Ti") => Unit::Tibi,
        s if s.starts_with('K') => Unit::Kilo,
        s if s.starts_with('M') => Unit::Mega,
        s if s.starts_with('G') => Unit::Giga,
        s if s.starts_with('T') => Unit::Tera,
        s if s.is_empty() ||
            s.ends_with('b') ||
            s.ends_with('B') ||
            s.to_lowercase().ends_with("bits") ||
            s.to_lowercase().ends_with("bytes") => Unit::Abs,
        // s if s == "" => 
        _ => panic!("Invalid unit"),
    };

    let base = match s {
        s if s.ends_with('b') ||
            s.to_lowercase().ends_with("bits") => Base::Bit,
        s if s.is_empty() ||
            s.ends_with('B') ||
            s.to_lowercase().ends_with("bytes") => Base::Byte,
        _ => panic!("Invalid base")
    };

    (unit, base)
}

pub fn parse_size(s: &str) -> Size {
    // let digits_end = s.find(|c: char| !c.is_numeric()).unwrap_or(s.len());
    let digits_end = s.find(|c: char| !c.is_numeric() && c != '.').unwrap_or(s.len());
    let size_str = &s[..digits_end];
    let value = if size_str.contains('.') {
        Value::Float(size_str.parse::<f64>().unwrap_or(0.0))
    } else {
        Value::Int(size_str.parse::<u64>().unwrap_or(0))
    };
    // let size = s[..digits_end].parse::<u64>().unwrap_or(0);
    let unit_base_str = &s[digits_end..];
    let (unit, base) = parse_unit_base(unit_base_str);
    Size {
        value,
        unit,
        base
    }
}
