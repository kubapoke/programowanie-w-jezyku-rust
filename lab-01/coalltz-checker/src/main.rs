use std::fs::File;
use std::io;
use std::io::Write;
use rand::Rng;

const MAX_ITER: u64 = 100;

fn get_power_array(x: u64) -> [u64; 10] {
    let mut result: [u64; 10] = [0; 10];

    result[0] = x;

    for i in 1..10 {
        result[i] = result[i-1] * x;
        println!("x^{} = {}", i + 1, result[i])
    }

    result
}

fn check_coalltz(arr: [u64; 10]) -> [bool; 10]
{
    let mut result = [false; 10];

    for elem in arr.iter().enumerate() {
        let mut x: u64 = *elem.1;

        for i in 0..MAX_ITER {
            if x % 2 == 0 {
                x = x / 2;
            }
            else {
                x = 3 * x + 1;
            }

            if x == 1 {
                result[elem.0] = true;
            }
        }
    }

    result
}

fn save_array_to_file(arr: [bool; 10]) -> () {
    let mut file = File::create("xyz.txt").expect("Could not create file");

    let mut content: [u8; 10] = [0; 10];
    for elem in arr.iter().enumerate() {
        content[elem.0] = match elem.1 {
            true => 1,
            false => 0,
        }
    }

    file.write_all(&content).expect("Could not write to file");
}

fn main() {
    loop {
        println!("Podaj liczbę");

        let mut string = String::new();

        io::stdin().read_line(&mut string).expect("Failed to read line");

        let mut x: u64 = match string.trim().parse::<u64>() {
            Ok(result) => result,
            Err(_) => break true,
        };

        if x == 0 {
            break false;
        }

        let rand = rand::thread_rng().gen_range(0..=5);
        x = x + rand;

        println!("{} generated, result is x := {} + {} = {}", rand, x - rand, rand, x);

        let arr = get_power_array(x);
        let arr = check_coalltz(arr);

        save_array_to_file(arr);
    };
}
