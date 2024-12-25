

use std::io::Read;
use std::io::Result;
use std::io::BufReader;
use std::fs::File;

fn main() -> Result<()>{
    let file = File::open("input2.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();

    let result = buf.lines().map(|line|{
        let split: Vec<_> = line.split(' ').collect();
        split 
    });

    let test:Vec<_> = result.collect();

    let mut answer = 0;
    for i in 0..200{
        println!("Index = {}", i+1);
        let test1 = is_acending(&test[i]);
        let test2 = is_decending(&test[i]);
        println!("{}", test1);
        println!("{}", test2);
        if test1 | test2{
            println!("Adding");
            answer = answer + 1;
        }
        println!("");
    }

    println!("Number of Safe = {}", answer);

    Ok(())
}

fn is_acending(array: &Vec<&str>) -> bool{
    for i in 0..array.len() - 1{
        let current = array[i];
        let next = array[i+1];
        // println!("Ascending checking if {} < {}", current, next);
        if current < next{
            let num1: i32 = current.parse().unwrap();
            let num2: i32 = next.parse().unwrap();
            let abs = (num1 - num2).abs();
            if (abs >= 4) | (abs == 0) {
                // println!("False becasue abs = {}", abs);
                return false;
            }
        }
        else{
            // println!("False because current !< next");
            return false;
        }
    }
    // println!("True");
    return true;
}

fn is_decending(array: &Vec<&str>) -> bool{
    for i in 0..array.len() - 1{
        let current = array[i];
        let next = array[i+1];
        // println!("Decedning checking if {} > {}", current, next);
        if current > next{
            let num1: i32 = current.parse().unwrap();
            let num2: i32 = next.parse().unwrap();
            let abs = (num1 - num2).abs();
            if (abs >= 4) | (abs == 0) {
                // println!("False becasue abs = {}", abs);
                return false;
            }
        }
        else{
            // println!("False because current !> next");
            return false;
        }
    }
    // println!("True");
    return true;
}
