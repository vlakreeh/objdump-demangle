use cpp_demangle::Symbol;
use std::{
    env,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    string::ToString,
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Enter a objdump of a dynamic symbol table");
    }

    let f = File::open(&args[1])?;
    let file = BufReader::new(&f);

    file.lines()
        .map(|line| line.unwrap())
        .map(|line| extract_symbol(line))
        .filter(|mangled_pair| mangled_pair.is_some())
        .map(|mangled_pair| mangled_pair.unwrap())
        .for_each(|mangled_pair| {
            let (address, mangled_symbol) = mangled_pair;

            match demangle(&mangled_symbol) {
                Ok(demangled_symbol) => println!("{} points to {}", address, demangled_symbol),
                Err(_) => eprintln!("Unable to demangle {}", mangled_symbol),
            }
        });

    Ok(())
}

fn extract_symbol(line: String) -> Option<(String, String)> {
    let underscore = line.find("_")?;
    let line: &str = line.as_str();

    let address = (&line[0..8]).to_string();
    let mangled = (&line[underscore..line.len()]).to_string();

    Some((address, mangled))
}

fn demangle(symbol: &String) -> Result<String, Box<dyn Error>> {
    Ok(Symbol::new(symbol)?.to_string())
}
