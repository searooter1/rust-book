fn main() {
    for num in 1..101 {
        match num {
            num if num % 15 == 0 => println!("FizzBuzz"),
            num if num % 3 == 0 => println!("Fizz"),
            num if num % 5 == 0 => println!("Buzz"),
            _ => println!("{}", num),
        };

        /*
        if num % 15 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", num);
        }
        */
    }
}