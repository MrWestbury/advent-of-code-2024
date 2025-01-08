use std::fs::read_to_string;

fn main() {
    println!("Day 1, Part 1");
    let data = read_input("../input.txt");
    println!("Len: {0}", data.len());
    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();
    for i in 0..data.len() {
        let mut line: String = data[i].clone();
        let remain: String = line.split_off(5);
        // println!("Remain {0}", remain);
        let num_str2: String = remain.clone().split_off(3);

        let num1: i32 = line.parse().unwrap();
        let num2: i32 = num_str2.parse().unwrap();
        column1.push(num1);
        column2.push(num2);
    }

    column1.sort();
    column2.sort();

    let mut total: i32 = 0;
    for j in 0..data.len() {
        let mut line_result: i32 = column2.get(j).unwrap() - column1.get(j).unwrap();
        if line_result < 0 {
            line_result = line_result * -1;
        }
        total += line_result;
        // println!("Line {0}: {1:?}, {2:?} = {3}", j, column1.get(j), column2.get(j), line_result);
    }

    println!("Total: {0}", total);
}

fn read_input(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}