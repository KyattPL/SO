use rand::prelude::*;
use std::fs;

const FRAMES_NO: i32 = 4;
const PAGE_MAX: i32 = 100;
const REQUESTS_NO: i32 = 100000;
const LOCAL_CHANCE: i32 = 25;

fn main() {
    let requests = generate_requests();
    fifo(requests);
}

fn generate_requests() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut current_request = 1;
    let mut requests = Vec::new();
    let mut chance: i32 = LOCAL_CHANCE;
    let mut length_of_local = 0;
    let mut lower_boundary = 0;
    let mut upper_boundary = 0;
    let mut stringified: String = String::new();

    while current_request <= REQUESTS_NO {
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
    write_to_file(stringified);
    requests
}

fn write_to_file(data: String) {
    fs::write("data.txt", data).expect("Can't write to the file");
}

fn fifo(requests: Vec<i32>) -> i32 {
    let mut page_fault_no = 0;
    let mut frames: [i32; FRAMES_NO as usize] = [0; FRAMES_NO as usize];
    page_fault_no
}
