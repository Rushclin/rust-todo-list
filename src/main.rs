use clap::Parser;
use serde_json::json;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    name: String,
    count: u8,
}

fn main() {
    println!("Hello, world!");

    let args = Args::parse();
    let json = json!({
        "name": "Takam"
    });

    println!("{:?}", args);
    println!("{:?}", json);
    println!("Le nom est {}", json["name"])
}
