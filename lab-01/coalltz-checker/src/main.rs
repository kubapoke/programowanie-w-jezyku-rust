use std::fs::File;
use std::io;
use std::io::Write;
use rand::Rng;

const MAX_ITER: u64 = 100;
const ARR_SIZE: usize = 10;
const START_VAL: u32 = 42;

fn get_power_array(x: u64) -> [u64; ARR_SIZE] {
    let mut result: [u64; ARR_SIZE] = [0; ARR_SIZE];

    result[0] = x;

    for i in 1..ARR_SIZE {
        result[i] = result[i-1] * x;
        println!("x^{} = {}", i + 1, result[i])
    }

    result
}

fn check_coalltz(arr: [u64; ARR_SIZE]) -> [bool; ARR_SIZE]
{
    let mut result = [false; ARR_SIZE];

    for elem in arr.into_iter().enumerate() {
        let mut x: u64 = elem.1;

        for _ in 0..MAX_ITER {
            if x.is_multiple_of(2) {
                x /= 2;
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

fn save_array_to_file(arr: [bool; ARR_SIZE]) -> bool {
    let mut file = File::create("xyz.txt").expect("Could not create file");

    let mut content: [u8; ARR_SIZE] = [0; ARR_SIZE];
    for elem in arr.into_iter().enumerate() {
        content[elem.0] = match elem.1 {
            true => b'1',
            false => b'0',
        }
    }

    file.write_all(&content).is_err()
}

fn main_loop() -> bool {
    loop {
        println!("Podaj liczbę");

        let mut string = String::new();

        io::stdin().read_line(&mut string).expect("Failed to read line");

        let mut x: u64 = match string.trim().parse::<u64>() {
            Ok(0) => break false,
            Ok(result) => result,
            Err(_) => break true,
        };

        let rand = rand::thread_rng().gen_range(0..=5);
        x += rand;

        println!("{} generated, result is x := {} + {} = {}", rand, x - rand, rand, x);

        let arr = get_power_array(x);
        let arr = check_coalltz(arr);

        if save_array_to_file(arr) {
            break true;
        }
    }
}

fn nested_loop(start: u32) -> (u32, f32) {
    let mut x = start;

    'label: loop {
        for i in 2..x {
            if x.is_multiple_of(i) && i > start {
                break 'label (x, i as f32 / start as f32);
            }
        }

        x += 1;
    }
}

fn main() {
    let result = main_loop();

    match result {
        true => {
            println!("Main loop exited due to error");
        },
        false => {
            println!("Main loop exited successfully");
        }
    };

    let (x, y) = nested_loop(START_VAL);
    println!("First number greater than {} with a proper divisor larger than it is {}; division value: {}", START_VAL, x, y);
}
