use std::io;
use std::time::Instant;

fn main() {
    println!("Fibonacci Number");
    println!("What is the depth? ");
    
    //Loop until integer is written to console
    let depth: u32 = loop{
        //Define string variable named data
        let mut data = String::new();
        //Read input into data
        io::stdin().read_line(&mut data)
            .expect("Failed to read line");
        //Parse data into u32
        //Rust parses the string to the return data format (u32)
        match data.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Veuillez saisir un chiffre");
            }
        }
    };
    //Start counter
    println!("#Counter Start");
    let now = Instant::now();

    //Print the result of the fibonacci algorithm
    println!("This result is fib({}) = {}",depth,fib(depth));

    //Display the elapsed time
    println!("#Counter End");   
    println!("#Elapsed Time: {}.{} s",
        now.elapsed().as_secs(),
        now.elapsed().as_millis()-(u128::from(now.elapsed().as_secs()*60)))

}

//Fibonacci function
fn fib(n:u32) -> u64 {
    if n<2 {
        let n : u64 = u64::from(n);
        n
    }else{
        let f1:u64 = fib(n-1);
        let f2:u64 = fib(n-2);
        f1+f2
    }
}

