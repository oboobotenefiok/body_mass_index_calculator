//Now using one input module against two, shortens it.

mod io_handler;
mod report;
mod body;

use body::Body;

fn main() {
    println!("What's your height in metres?");
    
    let height = io_handler::get_input();
    println!("You entered height as  {} \n
    Now, what's your weight in Kilograms?", height);
    
    let mass = io_handler::get_input();
    println!("You entered mass as {}", mass);
    
    let body_input = Body::new(mass, height);

    println!(
        "
    Your Body Mass Index is: {:.2} \n {}",
        body_input.bmi(), report::report(body_input.bmi())
    );
}

