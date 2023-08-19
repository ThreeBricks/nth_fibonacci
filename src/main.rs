use std::io;


fn main() {
    //just need to find the nth fibonacci number don't need to keep track of 0-nth-1 fibonacci numbers

    let mut two_behind: u32 = 0;
    let mut one_behind: u32 = 1;

    let mut nth = String::new();

    println!("Which number in the Fibonnaci sequence do you want?");

    io::stdin()
        .read_line(&mut nth)
        .expect("Failed to read line");

    let nth:u32 = match nth.trim().parse(){
        Ok(num) => num,
        Err(_) => 0,
    };

    if nth == 0 {
        println!("The {nth} Fibonacci number is {two_behind}");
        return;
    }
    if nth == 1 {
        println!("The {nth} Fibonacci number is {one_behind}");
        return;
    }


    for _number in 0..(nth-2) {
        let curr = two_behind + one_behind;
        two_behind = one_behind;
        one_behind = curr;
    }

    println!("The {nth} Fibonacci number is {one_behind}");

}
