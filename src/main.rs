use std::env;
mod d1;
mod d2;
mod d3;
mod d4;
mod d5;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please specify a day");
        return;
    }

    let day = args[1].as_str();

    match day {
        "1" => d1::solve(),
        "2" => d2::solve(),
        "3" => d3::solve(),
        "4" => d4::solve(),
        "5" => d5::solve(),
        // Add more days as you implement them
        _ => eprintln!("Day not implemented"),
    }
}
