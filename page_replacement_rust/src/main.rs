use rand::prelude::*;
use std::fs;
use std::io::stdin;

const FRAMES_NO: i32 = 10;
const PAGE_MAX: i32 = 30;
const REQUESTS_NO: i32 = 100_000;
const LOCAL_CHANCE: i32 = 25;

fn main() {
    let mut input: String = String::new();
    let requests: Vec<i32>;
    println!("1. Generate new requests");
    println!("2. Read requests from file");
    while input.trim() != "1" && input.trim() != "2" {
        std::io::stdin().read_line(&mut input).unwrap();
    }
    match input.trim().parse::<i32>().unwrap() {
        1 => {
            println!("How many requests: ");
            requests = generate_requests(get_request_no());
        }
        2 => requests = read_from_file(),
        _ => panic!("Unallowed state"),
    }
    println!("FIFO no. of page faults: {}", fifo(&requests));
    println!("OPT no. of page faults: {}", opt(&requests));
    println!("LRU no. of page faults: {}", lru(&requests));
    println!("RAND no. of page faults: {}", random(&requests));
    println!(
        "SC-LRU no. of page faults: {}",
        second_chance_lru(&requests)
    );
}

fn generate_requests(requests_no: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut current_request = 1;
    let mut requests = Vec::new();
    let mut chance: i32 = LOCAL_CHANCE;
    let mut length_of_local = 0;
    let mut lower_boundary = 0;
    let mut upper_boundary = 0;
    let mut stringified: String = String::new();

    let requests_no = if requests_no == 0 {
        REQUESTS_NO
    } else {
        requests_no
    };

    while current_request <= requests_no {
        if rng.gen_range(0, 100) <= chance && length_of_local == 0 {
            length_of_local = rng.gen_range(5, 100);
            chance = LOCAL_CHANCE;
            lower_boundary = rng.gen_range(1, PAGE_MAX / 2);
            upper_boundary = rng.gen_range(PAGE_MAX / 2 + 1, PAGE_MAX + 1);
        } else {
            chance += 2;
        }
        if length_of_local != 0 {
            length_of_local -= 1;
            let request = rng.gen_range(lower_boundary, upper_boundary + 1);
            stringified.push_str(&request.to_string());
            requests.push(request);
        } else {
            let request = rng.gen_range(1, PAGE_MAX + 1);
            stringified.push_str(&request.to_string());
            requests.push(request);
        }
        stringified.push_str("\n");
        current_request += 1;
    }
    write_to_file(String::from("data.txt"), stringified);
    requests
}

fn get_request_no() -> i32 {
    let mut input: String = String::new();
    loop {
        stdin().read_line(&mut input).unwrap();
        let num_of_requests = input.trim().parse::<i32>().unwrap();
        if num_of_requests > 0 {
            return num_of_requests;
        }
    }
}

#[allow(dead_code)]
fn generate_nonrandom_requests() -> Vec<i32> {
    let mut current_request = 1;
    let mut requests = Vec::new();
    let mut increment = 1;
    let mut stringified: String = String::new();

    while current_request <= REQUESTS_NO {
        let request = increment;
        increment += 1;
        if increment == FRAMES_NO + 1 {
            increment = 1;
        }
        stringified.push_str(&request.to_string());
        requests.push(request);
        stringified.push_str("\n");
        current_request += 1;
    }
    write_to_file(String::from("ascending_min.txt"), stringified);
    requests
}

fn write_to_file(file_name: String, data: String) {
    fs::write(file_name, data).expect("Can't write to the file");
}

fn read_from_file() -> Vec<i32> {
    let mut input: String = String::new();
    println!("Name of the file: ");
    stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let data = fs::read_to_string(input).expect("Unable to read the file");
    let mut requests: Vec<i32> = Vec::new();
    for line in data.lines() {
        let request: i32 = line.parse::<i32>().unwrap();
        requests.push(request);
    }
    requests
}

fn fifo(requests: &[i32]) -> i32 {
    let mut page_fault_no = 0;
    let mut frames: [i32; FRAMES_NO as usize] = [0; FRAMES_NO as usize];
    let mut initializer = 1;
    let mut current_request = FRAMES_NO;
    let mut oldest_frame = 0;

    while initializer <= FRAMES_NO {
        frames[(initializer - 1) as usize] = *requests.get((initializer - 1) as usize).unwrap();
        initializer += 1;
    }
    while current_request < REQUESTS_NO {
        if !frames.contains(requests.get(current_request as usize).unwrap()) {
            frames[oldest_frame] = *requests.get(current_request as usize).unwrap();
            page_fault_no += 1;
            oldest_frame += 1;
            if oldest_frame == FRAMES_NO as usize {
                oldest_frame = 0;
            }
        }
        current_request += 1;
    }

    page_fault_no
}

fn opt(requests: &[i32]) -> i32 {
    let mut page_fault_no = 0;
    let mut frames: [i32; FRAMES_NO as usize] = [0; FRAMES_NO as usize];
    let mut initializer = 1;
    let mut current_request = FRAMES_NO;

    while initializer <= FRAMES_NO {
        frames[(initializer - 1) as usize] = *requests.get((initializer - 1) as usize).unwrap();
        initializer += 1;
    }

    while current_request < REQUESTS_NO {
        if !frames.contains(requests.get(current_request as usize).unwrap()) {
            let mut max = 0;
            let mut index = 0;
            let mut max_index = 0;
            let mut scan_index = current_request;
            while index < FRAMES_NO {
                if frames[index as usize] == *requests.get(scan_index as usize).unwrap() {
                    if max < scan_index - current_request {
                        max = scan_index - current_request;
                        max_index = index;
                    }
                    index += 1;
                    scan_index = current_request;
                    continue;
                }
                scan_index += 1;
                if scan_index == REQUESTS_NO {
                    max_index = index;
                    break;
                }
            }
            frames[max_index as usize] = *requests.get(current_request as usize).unwrap();
            page_fault_no += 1;
        }
        current_request += 1;
    }

    page_fault_no
}

fn lru(requests: &[i32]) -> i32 {
    let mut page_fault_no = 0;
    let mut frames: [i32; FRAMES_NO as usize] = [0; FRAMES_NO as usize];
    let mut initializer = 1;
    let mut current_request = FRAMES_NO;

    while initializer <= FRAMES_NO {
        frames[(initializer - 1) as usize] = *requests.get((initializer - 1) as usize).unwrap();
        initializer += 1;
    }

    while current_request < REQUESTS_NO {
        if !frames.contains(requests.get(current_request as usize).unwrap()) {
            let mut min = REQUESTS_NO;
            let mut index = 0;
            let mut min_index = 0;
            let mut scan_index = current_request;
            while index < FRAMES_NO {
                if frames[index as usize] == *requests.get(scan_index as usize).unwrap() {
                    if min > scan_index {
                        min = scan_index;
                        min_index = index;
                    }
                    index += 1;
                    scan_index = current_request;
                    continue;
                }
                scan_index -= 1;
                if scan_index == -1 {
                    min_index = index;
                    break;
                }
            }
            frames[min_index as usize] = *requests.get(current_request as usize).unwrap();
            page_fault_no += 1;
        }
        current_request += 1;
    }

    page_fault_no
}

fn random(requests: &[i32]) -> i32 {
    let mut page_fault_no = 0;
    let mut frames: [i32; FRAMES_NO as usize] = [0; FRAMES_NO as usize];
    let mut initializer = 1;
    let mut current_request = FRAMES_NO;
    let mut rng = rand::thread_rng();

    while initializer <= FRAMES_NO {
        frames[(initializer - 1) as usize] = *requests.get((initializer - 1) as usize).unwrap();
        initializer += 1;
    }

    while current_request < REQUESTS_NO {
        if !frames.contains(requests.get(current_request as usize).unwrap()) {
            let temp = rng.gen_range(0, FRAMES_NO);
            frames[temp as usize] = *requests.get(current_request as usize).unwrap();
            page_fault_no += 1;
        }
        current_request += 1;
    }

    page_fault_no
}

fn second_chance_lru(requests: &[i32]) -> i32 {
    let mut page_fault_no = 0;
    let mut frames: Vec<i32> = vec![0; FRAMES_NO as usize];
    let mut frames_flags: Vec<bool> = vec![true; FRAMES_NO as usize];
    let mut initializer = 1;
    let mut current_request = FRAMES_NO;

    while initializer <= FRAMES_NO {
        frames[(initializer - 1) as usize] = *requests.get((initializer - 1) as usize).unwrap();
        initializer += 1;
    }

    while current_request < REQUESTS_NO {
        if !frames.contains(requests.get(current_request as usize).unwrap()) {
            let mut iterator = 0;
            while iterator < FRAMES_NO {
                if frames_flags[iterator as usize] {
                    frames_flags[iterator as usize] = false;
                    iterator += 1;
                    if iterator == FRAMES_NO {
                        iterator = 0;
                    }
                } else {
                    frames_flags.remove(iterator as usize);
                    frames_flags.push(true);
                    frames.remove(iterator as usize);
                    frames.push(*requests.get(current_request as usize).unwrap());
                    break;
                }
            }
            page_fault_no += 1;
        } else {
            let temp = *requests.get(current_request as usize).unwrap();
            let mut iterator = 0;
            while iterator < FRAMES_NO {
                if frames[iterator as usize] == temp {
                    frames_flags[iterator as usize] = true;
                }
                iterator += 1;
            }
        }
        current_request += 1;
    }

    page_fault_no
}
