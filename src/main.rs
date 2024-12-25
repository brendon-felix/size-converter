mod args;
use args::{Args, parse_size, parse_unit_base};
use clap::Parser;

mod convert;
use convert::{convert, Unit, Base};


fn main() {
    let args = Args::parse();
    let size = parse_size(&args.size);
    match &args.to {
        Some(s) => {
            let (unit, base) = parse_unit_base(s);
            println!("{}", convert(size, unit, base));
        }
        None => {
            println!("0x{:X}", convert(size, Unit::Abs, Base::Byte));
        }
    }
}
