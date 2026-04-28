use std::io;

pub fn get_input() -> f64 {
   
    loop { let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("We need a number, bro");

  let  input: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Please enter a valid number"); 
            continue;
        } ,
    };
   return input
    }
    
}
