use std::collections::HashMap;

fn main() {
    let v: Vec<u32> = vec![10, 10, 10];
    println!("The median is {}.", get_median(&v));
    println!("The mode is {}.", get_mode(&v));
}

fn get_median(nums: &Vec<u32>) -> u32 {
    // Factions are dropped from the median.
    let mut sum: u32 = 0;
    for num in nums {
        sum += num;
    }
    sum / nums.len() as u32
}

fn get_mode(nums: &Vec<u32>) -> u32 {
    let mut num_count: HashMap<u32, u32> = HashMap::new();
    for num in nums {
        let count = num_count.entry(*num).or_insert(0);
        *count += 1;
    }

    let mut mode: u32 = 0;
    let mut highest_count: u32 = 0;
    for (key, value) in num_count {
        if value > highest_count {
            mode = key;
            highest_count = value;
        }
    }
    mode
}
