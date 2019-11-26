use std::env;
use std::fs;
use std::error::Error;
use std::process;
use std::collections::HashMap;

fn main() {
    // let contents = rollup(args[1])?;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage:");
        println!("./top-traffic <filename>");
        process::exit(1);
    }
    if let Err(e) = rollup(&args) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}


pub fn rollup(args: &[String]) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args[1].clone())?;
    let mut traffics = HashMap::new();
    let mut flag = false;
    for line in contents.lines() {
        // let tmp = line.split(" ");
        let tmp:Vec<&str>= line.split(" ").collect();
        if !flag {
            if tmp[0] == "1" || tmp[0] == "2" || tmp[0] == "3" {
                println!("{} {} max:{}", tmp[1],tmp[2],tmp[3]);
                flag = true;
            }
        } else { flag = false; println!("{} max2:{}", tmp[0],tmp[2]);}
    }

    traffics.insert(String::from("Blue"), 10);
    traffics.insert(String::from("Yellow"), 50);

    for (key, value) in &traffics {
        println!("{}: {}", key, value);
    }
    Ok(())
}
