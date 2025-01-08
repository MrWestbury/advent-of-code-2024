use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    println!("Day 1, Part 1");
    let data = read_input("../input.txt");
    println!("Len: {0}", data.len());
    let mut column1:Vec<i32> = Vec::new();
    let mut datahash:HashMap<i32, i32> = HashMap::new();
    for i in 0..data.len() {
        let mut line: String = data[i].clone();
        let g_index = line.find(" ").unwrap();
        let remain: String = line.split_off(g_index);
        // println!("Remain {0}", remain);
        let num_str2: String = remain.clone().split_off(3);

        let num1: i32 = line.parse().unwrap();
        let num2: i32 = num_str2.parse().unwrap();
        
        column1.push(num1);
        datahash.entry(num2).and_modify(|count| *count += 1).or_insert(1);
    
    }

    let mut total: i32 = 0;
    for j in 0..column1.len() {
        let num = column1.get(j).unwrap();
        let count = datahash.entry(*num).or_default();
        let subtotal = *num * *count;
        println!("{0} * {1} = {2}", num, count, subtotal);
        total += subtotal;
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