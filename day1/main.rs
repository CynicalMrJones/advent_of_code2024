
use std::io::{self, Read};
use std::io::BufReader;
use std::fs::File;

fn part1() -> io::Result<()>{
    let file = File::open("input2.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();

    let result = buf.lines().map(|line| {
        let (num1, num2) = line.split_once("   ").unwrap();
        let num1: i32 = num1.parse().unwrap();
        let num2: i32 = num2.parse().unwrap();

        (num1, num2)
    });

    let (mut list1, mut list2): (Vec<i32>, Vec<i32>) = result.unzip();

    list1.sort();
    list2.sort();

    let mut total = 0;
    for number in 0..=list1.len()-1{
        total += list1[number].abs_diff(list2[number]);
    }

    println!("Total = {}",total);

    Ok(())
}

fn part2() -> io::Result<()>{
    let file = File::open("input2.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();

    let result = buf.lines().map(|line| {
        let (num1, num2) = line.split_once("   ").unwrap();
        let num1: i32 = num1.parse().unwrap();
        let num2: i32 = num2.parse().unwrap();

        (num1, num2)
    });

    let (list1, list2): (Vec<i32>, Vec<i32>) = result.unzip();


    let mut similarity = 0;
    for number in 0..=list1.len()-1{
        let left = list1[number];
        let mut multi = 0;
        for number1 in 0..=list2.len()-1 {
            if left == list2[number1] {
                multi += 1;
            } 
        }
        if multi != 0 {
            similarity = similarity + (left * multi);
        }
    }

    println!("Total = {}",similarity);

    Ok(())
}

fn main(){
    part1().unwrap();
    part2().unwrap();
}
