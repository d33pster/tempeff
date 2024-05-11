use argrust::{Arguments, FetchTypes, ArgumentDescription};
use tempeff::Files;
use rustypath::RPath;

fn main() {
    let mut args = Arguments::new(std::env::args().skip(1).collect());
    args.add("--scan", ArgumentDescription::new().short("-s"));
    args.add("--recursive", ArgumentDescription::new().short("-r"));
    
    args.analyse();

    let mut recursive = false;

    if args.ifarg_force("--recursive") {
        recursive = true;
    }

    if args.ifarg_force("--scan") {
        let path = RPath::new_from(&args.fetch("--scan", FetchTypes::Fetch(1)).get()[0]).expand();

        let mut dir = Files::new(path, recursive);

        dir.find_duplicates();

        println!("");

        dir.show();
    }
}