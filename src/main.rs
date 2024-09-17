use rand::Rng;
use clap::Parser;
#[derive(Parser)]

struct Cli {
    #[clap(short, long, default_value = "16")]
    length: usize,
}
fn main() {
    let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789()*&^%$#@!~";
    
    let args = Cli::parse();

    if args.length == 0 {
        println!("Length should be greater than 0");
        return;
    }

    let password: String = (0..args.length)
        .map(|_| {
            let index = rand::thread_rng().gen_range(0..characters.len());
            characters.chars().nth(index).unwrap()
        }).collect();
    
    println!("Generated password : {}", password);
}