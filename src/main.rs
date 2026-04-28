use std::io;

struct Body {
    mass: f64,
    height: f64,
}

/*Would shorten but that will defeat original intent to learn Struct & Impl */
impl Body {
    fn bmi(&self) -> f64 {
        (self.mass) / (self.height * self.height)
    }
}
fn main() {
    println!("What's your height in metres?");
    let height = get_input();
    println!("You entered height as  {} \n
    Now, what's your weight in Kilograms?", height);
    let mass = get_input();
    println!("You entered mass as {}", mass);
    
    let body_input = Body {
        mass,
        height,
    };

    println!(
        "
    Your Body Mass Index is: {:.2}",
        body_input.bmi()
    );
}

//Now using one input function against two, shortens it!

fn get_input() -> f64 {
   
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
