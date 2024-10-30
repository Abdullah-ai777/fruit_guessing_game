use std::io;
use rand::Rng; 

fn main() {
   
    let arr=["Apple","banana","mango","pineapple","orange"];
 
    loop {
        let mut randm = rand::thread_rng();
        let i: usize = randm.gen_range(0..arr.len());


       

       
        
        let mut input = String::new();
        
        println!("Try to guess the fruit names in this: {:?}",arr);

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
    
        if input == arr[i] {
            println!("You guessed it right!");
            break;
        } else {
            println!("Try again!");
        }
    }
}
