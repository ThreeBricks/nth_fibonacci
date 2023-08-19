use std::io;


fn main() {
    //just need to find the nth fibonacci number don't need to keep track of 0-nth-1 fibonacci numbers

    let mut two_behind: u32 = 0;
    let mut one_behind: u32 = 1;

    let mut nth = String::new();
    loop{
        println!("Which number in the Fibonnaci sequence do you want?");

        io::stdin
            .read_line(&mut nth)
            .expect("Failed to read line");

        let nth = match nth.trim().parse(){
            Ok(char) => char;
            Err(_) => continue;
        }
        break;
    }

    for number in (0..n-2) {
        let mut curr = two_behind + one_behind;
        two_behind = one_behind;
        one_behind = curr;
    }

    println!("The {nth} Fibonacci number is {one_behind}");

}
