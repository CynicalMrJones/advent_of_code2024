
use std::io::Read;
use std::io::Result;
use std::io::BufReader;
use std::fs::File;

fn main() -> Result<()>{
    let file = File::open("input1.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();

    let result = buf.lines().map(|line|{
        let split: Vec<_> = line.split(' ').collect();
        split 
    });

    let test:Vec<_> = result.collect();

    let mut answer = 0;
    for i in 0..6{
        let test1 = is_acending(&test[i]);
        let test2 = is_decending(&test[i]);
        if test1 | test2{
            answer = answer + 1;
        }
    }

    println!("Number of Safe = {}", answer);

    Ok(())
}

fn is_acending(array: &Vec<&str>) -> bool{
    for i in 0..array.len()-1{
        let current: i32 = array[i].parse().unwrap();
        let next: i32 = array[i+1].parse().unwrap();
        if current < next{
            let abs = (current - next).abs();
            if (abs >= 4) | (abs == 0) {
                return false;
            }
        }
        else{
            let mut test = array.clone();
            let index = test.iter().position(|x| *x == array[i+1]).unwrap();
            test.remove(index);
            let meme = is_acending_checker(&test);
            return meme;
        }
    }
    return true;
}

fn is_acending_checker(array: &Vec<&str>) -> bool{
    for i in 0..array.len()-1{
        let current: i32 = array[i].parse().unwrap();
        let next: i32 = array[i+1].parse().unwrap();
        if current < next{
            let abs = (current - next).abs();
            if (abs >= 4) | (abs == 0) {
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
    for i in 0..array.len()-1{
        let current: i32 = array[i].parse().unwrap();
        let next: i32 = array[i+1].parse().unwrap();
        if current > next{
            let abs = (current - next).abs();
            if (abs >= 4) | (abs == 0) {
                return false;
            }
        }
        else{
            let mut test = array.clone();
            let index = test.iter().position(|x| *x == array[i+1]).unwrap();
            test.remove(index);
            let meme = is_decending_checker(&test);
            return meme;
        }
    }
    return true;
}

fn is_decending_checker(array: &Vec<&str>) -> bool{
    for i in 0..array.len()-1{
        let current: i32 = array[i].parse().unwrap();
        let next: i32 = array[i+1].parse().unwrap();
        if current > next{
            let abs = (current - next).abs();
            if (abs >= 4) | (abs == 0) {
                return false;
            }
        }
        else{
            return false;
        }
    }
    return true;
}
