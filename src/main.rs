mod days;
mod helpers;

use days::*;

/**
 * Please don't change this lol - this whole program is probably very broken to breakage
 */
fn main() {
    // get the args from the input (read the README.md)
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "n" => new_day::run(args[2].as_str()), // pass the day input to the run function
            _ => println!("re read the README.md perhaps?"),
        }
    } else {
        println!("yeah read the README.md I reckon");
    }
}
