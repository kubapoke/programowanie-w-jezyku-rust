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

    let mut idx = 0;
    for elem in arr {
        let mut x: u64 = elem;

        for i in 0..MAX_ITER {
            if x % 2 == 0 {
                x = x / 2;
            }
            else {
                x = 3 * x + 1;
            }

            if x == 1 {
                result[idx] = true;
            }
        }

        idx = idx + 1;
    }

    result
}

fn main() {
    loop {
        println!("Podaj liczbe");

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

        println!("{} randomized, result is x = {}", rand, x);

        let arr = get_power_array(x);
        let arr = check_coalltz(arr);

        let mut file = File::create("xyz.txt").expect("Could not create file");

        let mut content = String::new();
        for elem in arr {
            content.push_str(&elem.to_string());
            content.push_str("\n");
        }

        todo!()
    };
}
