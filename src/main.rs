mod cursor;
mod generator;
mod operation;
mod optimizer;

use std::{
    env,
    error::Error,
    fs::{self, File},
    process,
};

use cursor::Cursor;
use generator::Generator;
use optimizer::Optimizer;

fn usage(path: &str) -> ! {
    eprintln!(
        r#"USAGE: {} source destination
 - source      : path to the source Brainfuck file,
 - destination : path to the output QBE IR file"#,
        path
    );
    process::exit(1)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let binary = args.next().unwrap();
    let Some(source) = args.next() else {
        usage(&binary);
    };
    let Some(output) = args.next() else {
        usage(&binary);
    };

    let input = fs::read_to_string(source).expect("source file not found");
    let mut file = File::create(output).expect("output file not found");

    let mut cursor = Cursor::new(&input);
    let mut optimizer = Optimizer::new(&mut cursor);

    let mut generator = Generator::new(&mut optimizer, &mut file);
    generator.generate()
}
