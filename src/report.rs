pub fn report(bmi: f64) -> String {
    let message = match bmi {
        // Looking at this again and it's cool using a match guard. Quite an overkill here till I change my mind.

        // We use match guards mostly for complex stuff though...
        bmi if bmi < 16.0 => "You are underweight. Please speak to a doctor or a dietician.",
        bmi if bmi >= 16.0 && bmi <= 25.0 => "Your weight is normal. Keep up the good work!",
        _ => "You're overweight, and should see to that. Visit the gym, see a doctor.", // Sounds Cruel :-)
    };
    message.to_string()
}
