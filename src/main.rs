use input::*;

fn main() {
    loop {   
        let shit: String = input_constrained("Input a number: ", |s| s.len() == 5);
        
        println!("{}", shit);
    }
}