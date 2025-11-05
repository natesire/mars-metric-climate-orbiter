use rayon::prelude::*;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Parallel map
    let doubled: Vec<i32> = numbers.par_iter()
        .map(|&x| x * 2)
        .collect();
    println!("{:?}", doubled);
    
    // Parallel filter
    let evens: Vec<&i32> = numbers.par_iter()
        .filter(|&&x| x % 2 == 0)
        .collect();
    println!("{:?}", evens);
    
    // Parallel for_each
    numbers.par_iter()
        .for_each(|x| println!("Processing: {}", x));
}
