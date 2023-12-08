use clap::Parser;
pub mod day1;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => day1::day1(),
        _ => println!("Day not found"),
    }
}

