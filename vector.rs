// Wirte a function that takes vector as an input and returns a vector with even values

// #Approch 1 (More memory efficient)

fn filter_even(v: &mut Vec<i32>) {
    let mut i = 0;
    while i < v.len() {
        if v[i] % 2 != 0 {
            v.remove(i);
        } else {
            i += 1;
        }
    } 
}

// #Approch 2 (Creates a new vector)

fn even_values(v: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for value in v {
        if value % 2 == 0 {
            new_vec.push(value);
        }
    }
    return new_vec;
}

fn main() {
    let mut vec_1 = Vec::new();
    vec_1.push(1);
    vec_1.push(2);
    vec_1.push(3);
    filter_even(&mut vec_1);

    println!("Updated Vector is {:?}", vec_1);

    let mut vec_2 = Vec::new();
    vec_2.push(1);
    vec_2.push(2);
    vec_2.push(3);
    vec_2.push(4);

    let new_vec = even_values(vec_2);
    println!("The new vector is {:?}", new_vec);
}