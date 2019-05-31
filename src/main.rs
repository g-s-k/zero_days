use std::env;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    if let Some(s) = env::args().skip(1).next() {
        println!("{}", zero_days::draw(&s));
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        println!("{}", zero_days::draw(&buf));
    }

    Ok(())
}
