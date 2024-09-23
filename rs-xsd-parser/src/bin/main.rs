use std::fs;

use clap::Parser;
use rs_xsd_parser::{flatten::datamodel_map::XsdDataModel, xsd::{attribute::AttributeGroup, Xsd}};
use yaserde::de::from_str;

// use rs_xsd_parser::{flatten::datamodel_map::XsdDataModel, xsd::{types::ComplexType, Xsd}};
#[derive(Parser)]
#[command(name = "program")]
#[command(about = "A simple file processing program")]
struct Cli {
    /// Input file
    #[arg(short, long)]
    input: String,

}

fn main() {
    let cli = Cli::parse();

    println!("Input file: {}", cli.input);

    if true { 
        let instance = Xsd::new_from_file("test",&cli.input).unwrap();
        println!("xmlns:{:#?} target_namespace:{:#?}",instance.namespace ,instance.schema.target_namespace);
        println!("{:#?}", instance);
        let data_model = XsdDataModel::build(&instance);

        if let Some((k,v)) = data_model.elements.first_key_value(){
            println!("flatten {:#?}",k);
            let e = v.flatten(&data_model);
            println!("{:#?}",e);
        }
    }else{
        let content = {
            let path = std::env::current_dir().unwrap();
            log::info!("The current directory is {}", path.display());

            fs::read_to_string(&cli.input).map_err(|e| e.to_string()).unwrap()
        };

        let  model: AttributeGroup = from_str(content.as_str()).unwrap();
        println!("{:#?}", model);


     // let mut reader = Deserializer::new_from_reader(content.as_bytes());    
     // let s = Sequence::deserialize(&mut reader);
     // println!("{:#?}",s);
    }


}
