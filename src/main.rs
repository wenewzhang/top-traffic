extern crate bytesize;
use bytesize::ByteSize;
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
    let mut last_size = String::from("");
    for line in contents.lines() {
        // let tmp = line.split(" ");
        let tmp:Vec<&str>= line.split(" ").collect();
        if !flag {
            if tmp[0] == "1" || tmp[0] == "2" || tmp[0] == "3" {
                println!("{} {} max:{}", tmp[1],tmp[2],tmp[3]);
                last_size = String::from(tmp[3]);
                flag = true;
            }
        } else {
            flag = false;
            println!("{} max2:{} {} Len:{}", tmp[0],tmp[2],last_size,last_size.len());
            //18.3KB -> KB
            let unitlz = &last_size.to_string()[last_size.len()-2..last_size.len()];
            //18.3KB -> 18.3
            let numlz = &last_size.to_string()[0..last_size.len()-2];
            let mut fnum = 0.0;
            if unitlz == "MB" {
                let cnumlz = numlz.parse::<f64>().unwrap();
                fnum = fnum + cnumlz*1024.0;
            } else if unitlz == "KB" {
                let cnumlz = numlz.parse::<f64>().unwrap();
                fnum = fnum + cnumlz;
            }
            let unitmp = &tmp[2].to_string()[tmp[2].len()-2..tmp[2].len()];
            let numtmp = &tmp[2].to_string()[0..tmp[2].len()-2];

            if unitmp == "MB" {
                let cnumtmp = numtmp.parse::<f64>().unwrap();
                fnum = fnum + cnumtmp*1024.0;
            } else if unitmp == "KB" {
                let cnumtmp = numtmp.parse::<f64>().unwrap();
                fnum = fnum + cnumtmp;
            }
            println!("{}-{}",fnum,unitlz);
            println!("{}-{}",unitmp,numtmp);

        }
    }

    traffics.insert(String::from("Blue"), 10);
    traffics.insert(String::from("Yellow"), 50);

    for (key, value) in &traffics {
        println!("{}: {}", key, value);
    }
    Ok(())
}

// fn calculate(unit:&str, num:&str)
