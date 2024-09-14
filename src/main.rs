use std::{collections::BTreeMap, str::FromStr};

use clap::{Parser, ArgAction, CommandFactory};
mod xsd;

#[derive(Parser)]
#[command(name = "program")]
#[command(about = "A simple file processing program")]
struct Cli {
    /// Input file
    #[arg(short, long)]
    input: String,

    /// Output file
    #[arg(short, long)]
    output: String,

}

fn main() {
    let cli = Cli::parse();

    // 打印输入和输出文件
    println!("Input file: {}", cli.input);
    println!("Output file: {}", cli.output);
    let bt = BTreeMap::new();

    let instance = xsd::Xsd::new_from_file("test",&cli.input,&bt).unwrap();
    print!("{:#?}",instance);
}
