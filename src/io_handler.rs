// use std::io;

pub fn get_input() -> f64 {
    loop {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("We need a number, bro");

        if let Ok(num) = input.trim().parse() {
            if num <= 0.0
            /* Must be decimal */
            {
                println!("Please input a real number");
                continue;
            }
            return num;
        }
        println!(
            "Please enter a valid
               number"
        );
        continue;
        /* This works, I wanted something more idiomatic on revisiting and I also handled the zero-negative edge  case up there...

          let input: f64 = match input.trim().parse() {
              Ok(num) => num,
              Err(_) => {
                  println!("Please enter a valid number");
                  continue;
              }
          };
        return  input; */
    }
}
