use std::fmt;

const KILO: u64 = 1_000;
const MEGA: u64 = 1_000_000;
const GIGA: u64 = 1_000_000_000;
const TERA: u64 = 1_000_000_000;
const KIBI: u64 = 1_024;
const MIBI: u64 = 1_048_576;
const GIBI: u64 = 1_073_741_824;
const TIBI: u64 = 1_099_511_627_776;

#[derive(Debug)]
pub enum Value {
    Int(u64),
    Float(f64),
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(val) => write!(f, "{}", *val as f64),
            Value::Float(val) => write!(f, "{}", val),
        }
    }
}
impl fmt::UpperHex for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(val) => write!(f, "{:X}", val),
            Value::Float(val) => write!(f, "{}", val),
        }
    }
}

#[derive(Debug)]
pub enum Unit {
    Abs,
    Kilo,
    Mega,
    Giga,
    Tera,
    Kibi,
    Mibi,
    Gibi,
    Tibi,
}
impl Unit {
    fn mult(&self) -> u64 {
        match self {
            Unit::Abs => 1,
            Unit::Kilo => KILO,
            Unit::Mega => MEGA,
            Unit::Giga => GIGA,
            Unit::Tera => TERA,
            Unit::Kibi => KIBI,
            Unit::Mibi => MIBI,
            Unit::Gibi => GIBI,
            Unit::Tibi => TIBI,
        }
    }
}
impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Abs => write!(f, ""),
            Unit::Kilo => write!(f, "K"),
            Unit::Mega => write!(f, "M"),
            Unit::Giga => write!(f, "G"),
            Unit::Tera => write!(f, "T"),
            Unit::Kibi => write!(f, "Ki"),
            Unit::Mibi => write!(f, "Mi"),
            Unit::Gibi => write!(f, "Gi"),
            Unit::Tibi => write!(f, "Ti"),
        }
    }
}

#[derive(Debug)]
pub enum Base {
    Bit,
    Byte
}
impl fmt::Display for Base {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Base::Bit => write!(f, "b"),
            Base::Byte => write!(f, "B"),
        }
    }
}

#[derive(Debug)]
pub struct Size {
    pub value: Value,
    pub unit: Unit,
    pub base: Base,
}
impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.value, self.unit, self.base)
    }
}
impl fmt::UpperHex for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self.value)
    }
}

fn to_bits(size: Size) -> u64 {
    let mult = match size.base {
        Base::Bit => size.unit.mult(),
        Base::Byte => size.unit.mult() * 8,
    };
    match size.value {
        Value::Int(val) => val * mult,
        Value::Float(val) => (val * (mult as f64)) as u64,
    }
}

fn to_bytes(size: Size) -> f64 {
    // dbg!(&size.unit);
    let mult = match size.base {
        Base::Bit => (size.unit.mult() as f64) / 8.0,
        Base::Byte => size.unit.mult() as f64,
    };
    // dbg!(mult);
    match size.value {
        Value::Int(val) => (val as f64) * mult,
        Value::Float(val) => val * mult,
    }
}

pub fn convert(size: Size, unit: Unit, base: Base) -> Size {
    let num = match base {
        Base::Bit => to_bits(size) as f64,
        Base::Byte => to_bytes(size),
    };
    let value = match unit {
        // Unit::Abs => Value::Int(num),
        _ => Value::Float((num as f64) / (unit.mult() as f64))
    };
    Size { value, unit, base }
}