use std::io;

fn main() {
    let mut input_temperature = String::new();
    let temperature: f32;
    let mut unit = String::new();
    loop {
        println!("Enter the temperature");
        io::stdin()
            .read_line(&mut input_temperature)
            .expect("Failed to read line");

        temperature = match input_temperature.trim().parse::<f32>() {
            Ok(num) => num * 1.0,
            Err(_) => {
                println!("You didn't enter a valid number");
                input_temperature.clear();
                continue;
            }
        };
        break;
    }

    loop {
        println!("Enter the unit ('F' or 'C')");
        match io::stdin().read_line(&mut unit) {
            Err(_) => {
                println!("Failed to read unity line");
                unit.clear();
                continue;
            }
            Ok(_a) => {}
        };
        unit = unit.trim().to_uppercase();

        match &unit[..] {
            "F" => {
                println!(
                    "you entered {}°F that equals {}°C",
                    temperature,
                    (temperature - 32.0) / 1.8
                );
                break;
            },
            "C" => {
                println!(
                    "you entered {}°C that equals {}°F",
                    temperature,
                    (temperature * 1.8) + 32.0
                );
                break;
            },
            _ => continue,
        };
    }
}
