fn main() {
    //By considering the terms in the Fibonacci sequence whose 
    //values do not exceed four million, find the sum of the even-valued terms
    
    // variable for running total
    let mut total = 0;
    let mut numbers = vec![];

    // I'm thinking three variables for the fib sequence
    let mut x = 1;
    let mut y = 2;
    let mut z = 0;

    while z < 4000000 {
        z = x + y;
        if z < 4000000 {
            numbers.push(z);
        }
        x = y;
        y = z;
    }

    //println!("{:?}",numbers);

    for n in &numbers {
        if n % 2 == 0 {
            total += n;
        }
    }

    println!("{}",total);
}
