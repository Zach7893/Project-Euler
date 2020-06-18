fn main() {
    // the goal of this problem is to find the sum of all the numbers
    // that are multiples of 3 or 5 between 1 and 1000

    let mut x = 1;
    let mut total = 0;
    while x < 1000 {
        if x % 3 == 0 || x % 5 == 0 {
            total += x;
        }
        x+=1;
    }
    println!("Answer is : {}",total);
}
