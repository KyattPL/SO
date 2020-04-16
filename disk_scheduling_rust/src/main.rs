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
    let (sstf_moves, starved_requests) = sstf(&mut v);
    println!("SSTF number of head moves: {}, this many requests have starved: {}", sstf_moves, starved_requests);
    let cscan_moves = cscan(&mut v);
    println!("C-SCAN number of head moves: {}", cscan_moves);
    let scan_moves = scan(&mut v);
    println!("SCAN number of head moves: {}", scan_moves);
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

fn sstf(requests: &mut Vec<Request>) -> (i32,i32) {
    let mut queue = Queue::new(vec![0]);
    let mut rng = rand::thread_rng();
    let mut request_no = 1;
    let mut head_position = 0;
    let mut head_moves = 0;
    let mut starving_requests = 0;

    loop {
        if rng.gen_range(1, 500) >= 494 && request_no < requests.len() as i32 {
            queue.push_request(request_no);
            queue.insertion_sort(head_position, requests);
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
                if processed_request.get_time_in_queue() > 20 {
                    starving_requests += 1;
                }
                queue.remove(0);
            }
        }

        let mut queue_size: i32 = queue.size() as i32;
        while queue_size > 1 {
            let queue_no: usize = queue.list[(queue_size - 1) as usize] as usize;
            requests.get_mut(queue_no).unwrap().add_time_in_queue();
            queue_size -= 1;
        }

        if requests.len() == (request_no as usize) && queue.is_empty() {
            break;
        }
    }

    (head_moves, starving_requests)
}

fn cscan(requests: &mut Vec<Request>) -> i32 {
    let mut rng = rand::thread_rng();
    let mut request_no = 0;
    let mut active_requests = 0;
    let mut head_position = 0;
    let mut head_moves = 0;
    let mut disk_array: [i32; BLOCK_SIZE as usize] = [0; BLOCK_SIZE as usize];

    loop {
        if rng.gen_range(1,500) >= 494 && request_no < requests.len() as i32 {
            let temp_block = requests.get(request_no as usize).unwrap().get_block_num();
            disk_array[temp_block as usize] += 1;
            request_no += 1;
            active_requests += 1;
        }

        if active_requests > 0 {
            head_position += 1;
            head_moves += 1;
            
            if head_position > BLOCK_SIZE-1 {
                head_position = 0;
            }

            if disk_array[head_position as usize] != 0 {
                active_requests -= disk_array[head_position as usize];
                disk_array[head_position as usize] = 0;
            }

            if active_requests == 0 && request_no == (requests.len() as i32){
                break;
            }
        }
    }
    head_moves
}

fn scan(requests: &mut Vec<Request>) -> i32 {
    let mut rng = rand::thread_rng();
    let mut request_no = 0;
    let mut active_requests = 0;
    let mut head_position = 0;
    let mut increment = 1;
    let mut head_moves = 0;
    let mut disk_array: [i32; BLOCK_SIZE as usize] = [0; BLOCK_SIZE as usize];

    loop {
         if rng.gen_range(1,500) >= 494 && request_no < requests.len() as i32 {
            let temp_block = requests.get(request_no as usize).unwrap().get_block_num();
            disk_array[temp_block as usize] += 1;
            request_no += 1;
            active_requests += 1;
        }

        if active_requests > 0 {
            head_position += increment;
            head_moves += 1;
            
            if head_position == BLOCK_SIZE-1 {
                increment = -1;
            } else if head_position == 0 {
                increment = 1;
            }

            if disk_array[head_position as usize] != 0 {
                active_requests -= disk_array[head_position as usize];
                disk_array[head_position as usize] = 0;
            }

            if active_requests == 0 && request_no == (requests.len() as i32){
                break;
            }
        }
    }
    head_moves
}
