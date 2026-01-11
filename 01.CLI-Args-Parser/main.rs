use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String, // usages : --name JohnDoe || -n JohnDoe || --name=--John || -n=--John

    #[arg(short, long, default_value_t = 1)]
    count: u8, //optional
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}