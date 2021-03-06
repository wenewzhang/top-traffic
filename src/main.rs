use std::env;
use std::fs;
use std::error::Error;
use std::process;
use std::collections::BTreeMap;

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
    let mut traffics = BTreeMap::new();
    let mut flag = false;
    let mut last_size = String::from("");
    for line in contents.lines() {
        // let tmp = line.split(" ");
        let tmp:Vec<&str>= line.split(" ").collect();
        if !flag {
            if tmp[0] == "1" || tmp[0] == "2" || tmp[0] == "3" || tmp[0] == "4" || tmp[0] == "5" || tmp[0] == "6" ||
            tmp[0] == "7" || tmp[0] == "8" || tmp[0] == "9" {
                // println!("Step1: {} {} max:{}", tmp[1],tmp[2],tmp[3]);
                last_size = String::from(tmp[3]);
                flag = true;
            }
        } else {
            flag = false;
            // println!("Step:{}",last_size);
            let mut fnum = 0.0;
            //18.3KB -> KB
            if last_size.len() > 2 {
                let unitlz = &last_size.to_string()[last_size.len()-2..last_size.len()];
                //18.3KB -> 18.3
                let numlz = &last_size.to_string()[0..last_size.len()-2];
                fnum = fnum + calculate(unitlz,numlz);
            }
            let unitmp = &tmp[2].to_string()[tmp[2].len()-2..tmp[2].len()];
            let numtmp = &tmp[2].to_string()[0..tmp[2].len()-2];
            fnum = fnum + calculate(unitmp,numtmp);
            let old_num = traffics.entry(tmp[0]).or_insert(0.0);
            *old_num += fnum;
        }
    }
// sort
    let mut traffics_vec: Vec<_> = traffics.iter().collect();
    traffics_vec.sort_by(|&(_, a), &(_, b)| a.partial_cmp(b).unwrap());
    for tv in traffics_vec.iter() {
        println!("{:?}",tv);
    }
    Ok(())
}

fn calculate(unit:&str, num:&str) -> f64 {
    let mut cnum = 0.0;
    if unit == "MB" {
        cnum = num.parse::<f64>().unwrap();
        cnum *= 1024.0;
    } else if unit == "KB" {
        cnum = num.parse::<f64>().unwrap();
    }
    cnum
}
