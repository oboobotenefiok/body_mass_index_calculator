//Now using one input module against two, shortens it.

mod body;
mod io_handler;
mod report;

use body::Body;

fn main() {
    println!("What's your height in metres?");

    let height = io_handler::get_input();
    println!(
        "You entered height as  {} \n
    Now, what's your weight in Kilograms?",
        height
    );

    let mass = io_handler::get_input();
    println!("You entered mass as {}", mass);

    // let body_input = Body::new(mass, height); 		// Let me try something here.
    let body_input = Body::new(mass, height).bmi();

    println!(
        "
    Your Body Mass Index is: {:.2} \n {}",
        /*  body_input.bmi(),
        report::report(body_input.bmi())*/
        // Yeah, it works!!!
        body_input,
        report::report(body_input)
    );
}

// I love this code review, lol :-)
