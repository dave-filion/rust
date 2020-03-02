use std::collections::HashMap;

#[derive(Debug)]
enum SpeadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {

    // vectors
    let _v: Vec<i32> = Vec::new();

    // for convienience:
    let mut my_vec = vec![1, 2, 3];

    print_vec(&my_vec);

    // update vector, mutable only
    my_vec.push(42);
    my_vec.push(99);

    print_vec(&my_vec);

    let third: &u32 = &my_vec[2];
    println!("second el is {}", third);

    // reading elements
    match my_vec.get(2) {
        Some(val) => println!("second el is {}", val),
        None => println!("there is no 2nd el")
    }

    // reading element that doesnt exist
    // let does_not_exist = &my_vec[100];

    // can have vec with different types from enum
    let row = vec![
        SpeadsheetCell::Int(23),
        SpeadsheetCell::Text(String::from("yo")),
        SpeadsheetCell::Float(10.12),
    ];

    // iterate over it
    for el in &row {
        println!("{:?}", el);
    }

    // hasmaps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 100);

    // using zip
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];

    let other_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("hasmap: {:?}", other_scores);

    // my_vec and v go out of scope here, drop elements inside them as well
}

fn print_vec(my_vec: &Vec<u32>) {
    for v in my_vec {
        println!("num -> {}", v);
    }
    println!("debug print: {:?}", my_vec)
}
