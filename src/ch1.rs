use std::io::BufRead;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    const FILE_PATH: &str = "data/ch1.csv";
    let mut left_min_heap = BinaryHeap::new();
    let mut right_min_heap = BinaryHeap::new();
    let file = std::fs::File::open(FILE_PATH).expect("Unable to open file");
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let values: Vec<_> = line.split("   ").collect();
        left_min_heap.push(Reverse(values[0].parse::<i32>().unwrap()));
        right_min_heap.push(Reverse(values[1].parse::<i32>().unwrap()));
    }
    let mut sum = 0;
    while !left_min_heap.is_empty() {
        let left = left_min_heap.pop().unwrap().0;
        let right = right_min_heap.pop().unwrap().0;
        sum += (right - left).abs();
    }
    println!("Sum of differences: {}", sum);
}
