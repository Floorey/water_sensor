use rand::Rng;
use std::io;

struct WaterLevelSensor;


impl WaterLevelSensor {
    fn is_water_low(&self) -> bool {
        let random_value = rand::thread_rng().gen_range(1..=100);
        random_value > 30
    }
    fn get_current_water_level(&self) -> u32 {
        rand::thread_rng().gen_range(1..=100)
    }
    fn release_water(&self) {
        println!("Output water!");
    }
}
struct CommandSender;

impl CommandSender  {
    fn send_command(&self, microcontroller_id: u32, command: &str) {
        println!(
            "Command to: '{}' microcontroller {} send.",
            command, microcontroller_id
        );
    }
}


fn main() {
    let rng = rand::thread_rng();
    let sensor1 = WaterLevelSensor;
    let sensor2 = WaterLevelSensor;
    let sender = CommandSender;


    loop {
        let water_level1 = sensor1.get_current_water_level();
        let water_level2 = sensor2.get_current_water_level();

        println!("Actual value: sensor1: {}", water_level1);
        println!("Actual value: sensor2: {}", water_level2);

        let mut input = String::new();
        println!("Please insert the sensor: 1 or 2. Valid inputs are 1 and 2. Type 'quit' to exit:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "quit" {
            break;
        } else if input == "1" || input == "2" {
            if input == "1" && sensor1.is_water_low() {
                sender.send_command(1, "output");
                sensor1.release_water();
            } else if input == "2" && sensor2.is_water_low() {
                sender.send_command(2, "output");
                sensor2.release_water();
            } else {
                println!("Invalid input: Please use 1 for sensor 1 and 2 for sensor 2.");
            }
        }
    }
}