use std::io;
use rand::Rng;

fn main() {
    println!("\nPlease input the length of your desired list:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Your size: {}", input);

    println!("Please input the largest possible entry in your list:");

    let mut largest = String::new();
    
    io::stdin()
        .read_line(&mut largest)
        .expect("Failed to read line");

    println!("The largest possible entry in your list: {}", largest);

    let length: usize = input.trim().parse().unwrap();
    let max: usize = largest.trim().parse().unwrap();
    let input_list = random_list(length, max);
    let sorted_list = beadsort(input_list);
    println!("The final sorted list is: {:?}\n", sorted_list)

}

fn beadsort(input_list: Vec<usize>) -> Vec<usize> {
    println!();
    let max = input_list.iter().max().unwrap(); // Find max value in list
    println!("Finding the max value in the input list: {:?}.", input_list);
    println!("Max Value: {:?}", max);
    println!();

    let mut transposed_list = vec![0; *max]; // Initialize transposed list as 0s to the length of max value
    println!("Initializing transposed_list to contain as many elements as the max value");
    println!("Transposed List: {:?}", transposed_list);
    println!();
    let mut return_list = Vec::new();

    println!("For each element (each 'column of beads') of the input list, \n'lay the beads flat' by incrementing as many elements of the \ntransposed list as the column is tall. \nThese will accumulate atop previous additions.");
    println!();
    for num in input_list.clone() {
        println!("Looking at entry {:?}", num);
        println!();
        println!("\tInitial Transposed List: {:?}", transposed_list);
        let new_transposed_list: Vec<isize> = transposed_list // transpose the list
            .iter()
            .enumerate()
            .map(|x| {
                let i = x.0; 
                let n = x.1;
                if i < num {*n + 1} else {*n}
            })
            .collect();
        transposed_list.clear();
        transposed_list.extend(&new_transposed_list);
        println!("\tUpdated Transposed List: {:?}", transposed_list);
        println!();
    }
    println!("Final Transposed List: {:?}", transposed_list);
    println!();

    println!("We've now dropped the beads. To de-transpose, we count the \n'bottommost row' of dropped beads, then mimic removing this\nrow by subtracting 1 from each 'column' of the transposed list. \nWhen a column does not reach high enough for the current row, \nits value in transposed_list will be <= 0.");
    println!();

    for _ in input_list.clone() {
        println!("Subtracting 1 from all possible values in Transposed List: {:?} and pushing onto Return List: {:?}", transposed_list, return_list);
        return_list.push(
            transposed_list
                .iter()
                .fold(0, |a, n| if n > &0 {a + 1} else {a}) // push into return_list the number of positive integers in T
        );
        println!("Return List is now: {:?}", return_list);
        let new_transposed_list: Vec<isize> = transposed_list // subtract 1 from all values in T
            .iter()
            .map(|x| x - 1)
            .collect();
        transposed_list.clear();
        transposed_list.extend(&new_transposed_list);
        println!("Transposed List is now: {:?}", transposed_list);
        println!();
    }
    println!("Transposed list no longer contains positive integers. Sort is finished.");
    println!();
    return_list
}

fn random_list(length: usize, max: usize) -> Vec<usize> { // Generates a random list given the user's desired list length
    let mut return_list = Vec::new();
    for _ in 0..(length) {
        let mut rng = rand::thread_rng();
        let rnum = rng.gen_range(1, max + 1);
        return_list.push(rnum);
    }
    println!("Content of randomized vector for program: {:?}", return_list);
    return_list
}