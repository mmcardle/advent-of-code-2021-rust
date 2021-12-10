use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

fn main() {
    with_file(String::from("day3_test_input"), 22, 9);
    with_file(String::from("day3_input"), 218, 3877);
}

fn with_file(f: String, expected: u32, expected2: u32) {
    let file = File::open(f).expect("no such file");
    let buf = BufReader::new(file).lines();
    let test_data = buf.collect::<Result<Vec<_>, _>>().expect("error reading file");

    let empty: Vec<String> = vec![];

    let gamma = process(test_data.clone(), empty.clone(), 0, false);
    println!("=== GAMMA {} ===", gamma);
    assert!(gamma == expected, "gamma is {}, not the expected {}", gamma, expected);
    
    let epsilon = process(test_data.clone(), empty.clone(), 0, true);
    println!("=== EPSILON {} ===", epsilon);
    assert!(epsilon == expected2, "epsilon is {}, not the expected {}", epsilon, expected2 );

    println!("Result {} * {} = {}", gamma, epsilon, gamma * epsilon);
}

fn process(data: Vec<String>, mut result: Vec<String>, i: usize, inverse: bool) -> u32 {

    let len_result = result.len();
    let len_data = data[0].len();
    if len_result == len_data {
        if inverse {
            let inverted_string:String = result.join("").chars()
                .map(|x| match x { 
                    '0' => '1', 
                    '1' => '0', 
                    _ => x
                }).collect();
            let intval = isize::from_str_radix(&inverted_string, 2).unwrap();
            println!("binary inverse is {} => {}", inverted_string, intval);
            return intval as u32;
        } else {
            let intval = isize::from_str_radix(&result.join(""), 2).unwrap();
            println!("binary is {} => {}", &result.join(""), intval);
            return intval as u32;
        }
    }

    let mut zero_vec: Vec<String> = vec![];
    let mut ones_vec: Vec<String> = vec![];

    for line in data.clone() {
        let digit = line.chars().nth(i).unwrap();
        if digit == '0' {
            zero_vec.push(line);
        } else if digit == '1' {
            ones_vec.push(line);
        } else {
            panic!("invalid digit {}", digit);
        }
    }

    if zero_vec.len() >= ones_vec.len() {
        result.push(String::from("0"));
    } else {
        result.push(String::from("1"));
    }

    return process(data, result, i + 1, inverse);
}