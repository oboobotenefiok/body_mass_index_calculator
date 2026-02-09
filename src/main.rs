use std::io;

struct Body {
    mass: f64,
    height: f64,
}
impl Body {
    fn bmi(&self) -> f64 {
        (self.mass) / (self.height * self.height)
    }
}
fn main() {
    println!("What's your height in metres?");
    let mut height = String::new();
    io::stdin()
        .read_line(&mut height)
        .expect("We need a number, bro");

    let height: f64 = match height.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Please enter a valid number"); 
            return;
        } ,
    };

    println!("You entered {} \n
    Now, what's your weight in Kilograms?", height);
    let mut mass = String::new();
    io::stdin()
        .read_line(&mut mass)
        .expect("We need a number, bro");
    let mass: f64 = match mass.trim().parse() {
        Ok(num) => num,
        Err(_) => { println!("Please enter a valid number");
            return;
        }, // I need to handle this
    };
println!("You entered {}", mass);
    let body_input = Body {
        mass: mass,
        height: height,
    };

    println!(
        "
    Your Body Mass Index is: {:.2}",
        body_input.bmi()
    );
}
