use std::io::BufRead;
use std::collections::LinkedList;
use std::collections::HashMap;

fn main() {
    const FILE_PATH: &str = "data/ch1.csv";
    let mut elements_at_left_row = LinkedList::new();
    let mut similarity_count_hash = HashMap::new();
    let file = std::fs::File::open(FILE_PATH).expect("Unable to open file");
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let values_in_line: Vec<_> = line.split("   ").map(String::from).collect();
                elements_at_left_row.push_back(values_in_line[0].clone());
                similarity_count_hash
                    .entry(values_in_line[1].clone())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
                
            }
            Err(line) => {
                println!("Error with line: {}", line);
            }
        }
    }
    
    let mut sum_with_similarity = 0;
    for element in elements_at_left_row.iter() {
        if let Some(&count) = similarity_count_hash.get(element) {
            sum_with_similarity += count * element.parse::<i32>().unwrap();
        }
    }
    
    println!("{}", sum_with_similarity);
}
