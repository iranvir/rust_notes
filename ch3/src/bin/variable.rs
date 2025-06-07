fn main() {
    let x = 5;
    let x = x + 1;
    println!{"The value of x is: {x}"};
    {
        let x = 2 * x;
        println!{"The value of x within the scope is: {x}"};
    }
    println!{"The value of x out getting out of scope is: {x}"};
}
