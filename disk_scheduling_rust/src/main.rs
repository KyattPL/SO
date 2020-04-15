mod queue;
mod request;

use queue::Queue;
use rand::prelude::*;
use request::Request;
use std::fs;

const BLOCK_SIZE: i32 = 200;

fn main() {
    let mut v: Vec<Request> = generate_requests(100000);
    let fcfs_moves = fcfs(&mut v);
    println!("FCFS number of head moves: {}", fcfs_moves);
}

fn generate_requests(mut num_of_requests: i32) -> Vec<Request> {
    let mut requests: Vec<Request> = vec![];
    let mut rng = thread_rng();
    let mut string = String::from("");
    while num_of_requests != 0 {
        let temp_block = rng.gen_range(0, BLOCK_SIZE);
        let temp_realtime = rng.gen_range(1, 100);
        let mut is_realtime: bool = false;
        let mut time_to_handle: i32 = 0;
        if temp_realtime <= 10 {
            is_realtime = true;
            time_to_handle = rng.gen_range(10, 100);
        }
        string.push_str(&temp_block.to_string());
        string.push_str(" ");
        string.push_str(&time_to_handle.to_string());
        string.push_str("\n");
        requests.push(Request::new(temp_block, is_realtime, 0, time_to_handle));
        num_of_requests -= 1;
    }
    write_to_file(string, &String::from("data_rand.txt"));
    requests
}

fn write_to_file(data: String, path: &String) {
    fs::write(path, data).expect("Unable to write to the file");
}

fn fcfs(requests: &mut Vec<Request>) -> i32 {
    let mut queue = Queue::new(vec![0]);

    let mut rng = rand::thread_rng();
    let mut head_moves: i32 = 0;
    let mut request_no: i32 = 1;
    let mut head_position: i32 = 0;

    loop {
        if rng.gen_range(1, 500) >= 494 && request_no < requests.len() as i32 {
            queue.push_request(request_no);
            request_no += 1;
        }

        if queue.size() > 0 {
            let first_in_queue: usize = queue.list[0] as usize;
            let processed_request = requests.get_mut(first_in_queue).unwrap();
            let current_block = processed_request.get_block_num();
            if current_block > head_position {
                head_position += 1;
                head_moves += 1;
            } else if current_block < head_position {
                head_position -= 1;
                head_moves += 1;
            } else {
                queue.remove(0);
                //println!("queu size: {}", queue.size());
            }
        }

        if requests.len() == (request_no as usize) && queue.is_empty() {
            break;
        }
    }
    head_moves
}

fn sstf(requests: &mut Vec<Request>) -> i32 {
    let mut queue = Queue::new(vec![0]);
    let mut rng = rand::thread_rng();
    let mut request_no = 1;
    let mut head_position = 0;
    let mut head_moves = 0;
    let mut starving_requests = 0;

    loop {
        if rng.gen_range(1, 500) >= 494 && request_no < requests.len() as i32 {
            queue.push_request(request_no);
            request_no += 1;
        }
    }

    head_moves
}
