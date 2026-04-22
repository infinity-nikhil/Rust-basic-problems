// Write the logic to first filter all odd values then double each value and create a new vector 
//Two things to learn from here: Use of an itarator and the conversion of an itraor back an Vector 
fn main() {
    let vec_1 = vec![1, 2, 3, 4, 5];
    let vec_2 = vec![1, 2, 3, 4];

    let iter = vec_2.iter().filter(|x| *x % 2 != 0).map(|x| x * 2);
    let v2: Vec<i32> = iter.collect(); //use of collect 
    println!("{:?}", v2);

    let iter_1 = vec_1.iter().filter(|x| *x % 2 != 0);
    let iter_2 = iter_1.map(|x| x * 2);

    for val in iter_2 {   //itarotrs are lazy you have to call them over 
        println!("{}", val);
    }
}