use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn generate_workout(intensity: u32, randome_number: u32) {
    let expensive_closure = |num| {
        println!("slow calc");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today, do {} pushups", expensive_result);
        println!("Next, do {} situps", expensive_result);
    } else {
        print!("take a break todaay");
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
