use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;

    for line in reader.lines() {
        let mut nums = vec![];
        let mut num_count = 0;
        for c in line?.chars() {
           if c.is_numeric() {
              nums.push(c);  
              num_count += 1;
           } 
        }

        let mut str = String::from("");
        str.push(nums[0]);
        str.push(nums[num_count - 1]);

        sum += str.parse::<u32>().unwrap();
    }

    println!("{}", sum);

    return Ok(());
}
