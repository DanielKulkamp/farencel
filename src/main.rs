use std::io;

fn main() {
    let mut input_temperature = String::new();
    let temperature: f32;
    let mut unit = String::new();
    loop {
        println!("Enter the temperature");
        input_temperature = String::new();
        io::stdin()
            .read_line(&mut input_temperature)
            .expect("Failed to read line");

        temperature = match input_temperature.trim().parse::<f32>() {
            Ok(num) => num * 1.0,
            Err(_) => {
                println!("You didn't enter a valid number");
                continue;
            }
        };
        break;
    }

    loop {
        let f = String::from("F");
        let c = String::from("C");

        println!("Enter the unit ('F' or 'C')");
        unit = String::new();
        match io::stdin().read_line(&mut unit) {
            Err(_) => {
                println!("Failed to read unity line");
                continue;
            }
            Ok(_a) => {}
        };
        unit = unit.trim().to_string();
        if unit.eq_ignore_ascii_case(&f) {
            println!(
                "you entered {}°F that equals {}°C",
                temperature,
                (temperature - 32.0) / 1.8
            );
            break;
        }
        if unit.eq_ignore_ascii_case(&c) {
            println!(
                "you entered {}°C that equals {}°F",
                temperature,
                (temperature * 1.8) + 32.0
            );
            break;
        }
    }
}
