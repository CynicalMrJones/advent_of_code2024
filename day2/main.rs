

use std::io::Read;
use std::io::Result;
use std::io::BufReader;
use std::fs::File;

fn main() -> Result<()>{
    let file = File::open("input1.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();
    
    println!("{}", buf);

    let result = buf.lines().map(|line|{
        let split: Vec<_> = line.split(' ').collect();
        split 
    });

    let test:Vec<_> = result.collect();

    let mut answer = 0;
    for i in 0..=test.len()-1{
        let test1 = is_acending(&test[i]);
        let test2 = is_decending(&test[i]);
        if (test1 == true) | (test2 == true){
            answer += 1;
        }
    }

    println!("Number of Safe = {}", answer);

    Ok(())
}

fn is_acending(array: &Vec<&str>) -> bool{
    for i in 0..array.len() - 1{
        let current = array[i];
        let next = array[i+1];
        if current < next{
            let num1: i32 = current.parse().unwrap();
            let num2: i32 = next.parse().unwrap();
            if (num1 - num2).abs() >= 4{
                return false;
            }
        }
        else{
            return false;
        }
    }
    return true;
}

fn is_decending(array: &Vec<&str>) -> bool{
    for i in 0..array.len() - 1{
        let current = array[i];
        let next = array[i+1];
        if current > next{
            let num1: i32 = current.parse().unwrap();
            let num2: i32 = next.parse().unwrap();
            if (num1 - num2).abs() >= 4{
                return false;
            }
        }
        else{
            return false;
        }
    }
    return true;
}
