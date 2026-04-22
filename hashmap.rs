// Convert a vector of tuple into a hasmap 
use std::collections::HashMap; //Import the hashmap from standard library 

fn convert_tuple_to_hashmap(v: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in v {
        hm.insert(key, value);
    }
    return hm;
}

fn main() {
    let input_vec = vec![
        (String::from("Nikhil"), 20),
        (String::from("Rohit"), 18),
    ];

    let converted_hashmap = convert_tuple_to_hashmap(input_vec);

    println!("The hashmap is {:?}", converted_hashmap);
}