mod queue;
mod request;

use queue::Queue;
use rand::prelude::*;
use request::Request;
use std::fs;
use std::io::*;

const BLOCK_SIZE: i32 = 200;
const SSTF_STARVING: i32 = 100;
const RT_PERCENTAGE: i32 = 10;
const RT_TIME: (i32, i32) = (10, 100);
const RNG_CHANCE: i32 = 494;
const RNG_MAX: i32 = 500;

fn main() {
    let mut input: String = String::new();
    let mut v: Vec<Request>;
    println!("1. Generate new requests");
    println!("2. Read requests from file");
    while input.trim() != "1" && input.trim() != "2" {
        std::io::stdin().read_line(&mut input).unwrap();
    }
    match input.trim().parse::<i32>().unwrap() {
        1 => {
            println!("How many requests: ");
            v = generate_requests(get_request_no());
        }
        2 => v = read_from_file(),
        _ => panic!("Unallowed state"),
    }
    let mut output = String::from("");
    let fcfs_moves = fcfs(&mut v);
    let fcfs_out = format!("FCFS number of head moves: {}", fcfs_moves);
    println!("{}", fcfs_out);
    output.push_str(&fcfs_out);
    output.push('\n');
    let (sstf_moves, starved_requests) = sstf(&mut v);
    let sstf_out = format!(
        "SSTF number of head moves: {}, this many requests have starved: {}",
        sstf_moves, starved_requests
    );
    println!("{}", sstf_out);
    output.push_str(&sstf_out);
    output.push('\n');
    let cscan_moves = cscan(&mut v, true);
    let cscan_out = format!("C-SCAN number of head moves: {}", cscan_moves);
    println!("{}", cscan_out);
    output.push_str(&cscan_out);
    output.push('\n');
    let scan_moves = scan(&mut v);
    let scan_out = format!("SCAN number of head moves: {}", scan_moves);
    println!("{}", scan_out);
    output.push_str(&scan_out);
    output.push('\n');
    let edf_info = edf(&mut v);
    let edf_out = format!(
        "EDF number of head moves: {}, RT handled: {}, RT starved: {}",
        edf_info.0, edf_info.1, edf_info.2
    );
    println!("{}", edf_out);
    output.push_str(&edf_out);
    output.push('\n');
    let fdscan_info = fdscan(&mut v);
    let fdscan_out = format!(
        "FD-SCAN number of head moves: {}, RT handled: {}, RT starved: {}",
        fdscan_info.0, fdscan_info.1, fdscan_info.2
    );
    println!("{}", fdscan_out);
    output.push_str(&fdscan_out);
    output.push('\n');
    write_to_file(output, &String::from("results.txt"));
}

fn generate_requests(mut num_of_requests: i32) -> Vec<Request> {
    let mut requests: Vec<Request> = vec![];
    let mut rng = thread_rng();
    let mut string = String::from("");
    while num_of_requests != 0 {
        let temp_block = rng.gen_range(0, BLOCK_SIZE);
        let temp_realtime = rng.gen_range(1, 100);
        let mut is_realtime: bool = false;
        let time_to_handle: i32 = if temp_realtime <= RT_PERCENTAGE {
            is_realtime = true;
            rng.gen_range(RT_TIME.0, RT_TIME.1)
        } else {
            0
        };
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

#[allow(dead_code)]
fn generate_nonrandom_requests(num_of_requests: i32) -> Vec<Request> {
    let mut requests: Vec<Request> = Vec::new();
    let mut output: String = String::new();
    let min = 0;
    let max = BLOCK_SIZE - 1;
    let mut counter = max;
    //let mut rng = rand::thread_rng();
    let mut iterator = num_of_requests;
    while iterator != 0 {
        //let temp_block = rng.gen_range(0, BLOCK_SIZE);
        requests.push(Request::new(counter, false, 0, 0));
        output.push_str(&counter.to_string());
        output.push_str(" ");
        output.push_str("0");
        output.push_str("\n");
        counter -= 1;
        if counter == min - 1 {
            counter = max;
        }
        iterator -= 1;
    }
    write_to_file(output, &String::from("general_descending.txt"));
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

fn write_to_file(data: String, path: &str) {
    fs::write(path, data).expect("Unable to write to the file");
}

fn read_from_file() -> Vec<Request> {
    let mut input: String = String::new();
    println!("Name of the file: ");
    stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let data = fs::read_to_string(input).expect("Unable to read the file");
    let mut requests: Vec<Request> = Vec::new();
    for line in data.lines() {
        let request: Vec<&str> = line.split(' ').collect();
        let is_rt: bool = request[1] != "0";
        let request_int: (i32, i32) = (
            request[0].parse::<i32>().unwrap(),
            request[1].parse::<i32>().unwrap(),
        );
        requests.push(Request::new(request_int.0, is_rt, 0, request_int.1));
    }
    requests
}

fn fcfs(requests: &mut Vec<Request>) -> i32 {
    let mut queue = Queue::new(vec![0]);

    let mut rng = rand::thread_rng();
    let mut head_moves: i32 = 0;
    let mut request_no: i32 = 1;
    let mut head_position: i32 = 0;

    loop {
        if rng.gen_range(1, RNG_MAX) >= RNG_CHANCE && request_no < requests.len() as i32 {
            queue.push_request(request_no);
            request_no += 1;
        }

        if queue.size() > 0 {
            let first_in_queue: usize = queue.list[0] as usize;
            let processed_request = requests.get(first_in_queue).unwrap();
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

fn sstf(requests: &mut Vec<Request>) -> (i32, i32) {
    let mut queue = Queue::new(vec![0]);
    let mut rng = rand::thread_rng();
    let mut request_no = 1;
    let mut head_position = 0;
    let mut head_moves = 0;
    let mut starving_requests = 0;

    loop {
        if rng.gen_range(1, RNG_MAX) >= RNG_CHANCE && request_no < requests.len() as i32 {
            queue.push_request(request_no);
            queue.insertion_sort(head_position, requests);
            request_no += 1;
        }

        if queue.size() > 0 {
            let first_in_queue: usize = queue.list[0] as usize;
            let processed_request = requests.get(first_in_queue).unwrap();
            let current_block = processed_request.get_block_num();
            if current_block > head_position {
                head_position += 1;
                head_moves += 1;
            } else if current_block < head_position {
                head_position -= 1;
                head_moves += 1;
            } else {
                if processed_request.get_time_in_queue() > SSTF_STARVING {
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

fn cscan(requests: &mut Vec<Request>, is_c: bool) -> i32 {
    let mut rng = rand::thread_rng();
    let mut request_no = 0;
    let mut active_requests = 0;
    let mut head_position = 0;
    let mut head_moves = 0;
    let mut disk_array: [i32; BLOCK_SIZE as usize] = [0; BLOCK_SIZE as usize];
    let mut increment = 1;

    loop {
        if rng.gen_range(1, RNG_MAX) >= RNG_CHANCE && request_no < requests.len() as i32 {
            let temp_block = requests.get(request_no as usize).unwrap().get_block_num();
            disk_array[temp_block as usize] += 1;
            request_no += 1;
            active_requests += 1;
        }

        if active_requests > 0 {
            if is_c {
                if head_position > BLOCK_SIZE - 1 {
                    head_position = 0;
                }
            } else {
                if head_position == BLOCK_SIZE - 1 {
                    increment = -1;
                } else if head_position == 0 {
                    increment = 1;
                }
            }

            if disk_array[head_position as usize] != 0 {
                active_requests -= disk_array[head_position as usize];
                disk_array[head_position as usize] = 0;
            }

            if active_requests == 0 && request_no == (requests.len() as i32) {
                break;
            }

            head_position += increment;
            head_moves += 1;
        }
    }
    head_moves
}

fn scan(requests: &mut Vec<Request>) -> i32 {
    cscan(requests, false)
}

fn edf(requests: &mut Vec<Request>) -> (i32, i32, i32) {
    let mut rng = rand::thread_rng();
    let mut queue_basic = Queue::new(vec![]);
    let mut queue_rt = Queue::new(vec![]);
    let mut head_moves = 0;
    let mut head_position = 0;
    let mut request_no = 0;
    let mut realtime_handled = 0;
    let mut realtime_starved = 0;

    loop {
        if rng.gen_range(1, RNG_MAX) >= RNG_CHANCE && request_no < requests.len() as i32 {
            if requests.get(request_no as usize).unwrap().is_realtime() {
                queue_rt.push_request(request_no);
                queue_rt.rt_insertion_sort(requests);
            } else {
                queue_basic.push_request(request_no);
            }
            request_no += 1;
        }

        if queue_basic.size() > 0 || queue_rt.size() > 0 {
            if queue_rt.size() > 0 {
                let first_in_queue = queue_rt.list[0] as usize;
                let processed_request = requests.get(first_in_queue).unwrap();
                let current_block = processed_request.get_block_num();
                if current_block > head_position {
                    head_position += 1;
                    head_moves += 1;
                } else if current_block < head_position {
                    head_position -= 1;
                    head_moves += 1;
                } else {
                    realtime_handled += 1;
                    queue_rt.remove(0);
                }
                let mut queue_size = queue_rt.size() as i32;
                let mut i = 0;
                while queue_size > i {
                    let temp_index = queue_rt.list[i as usize];
                    let temp_request = requests.get(temp_index as usize).unwrap();
                    if temp_request.time_remaining() == 0 {
                        queue_rt.remove(i as usize);
                        queue_size -= 1;
                        realtime_starved += 1;
                        continue;
                    }
                    i += 1;
                }

                let mut queue_size = queue_rt.size() as i32;
                while queue_size > 0 {
                    let temp_index = queue_rt.list[(queue_size - 1) as usize];
                    let temp_request = requests.get_mut(temp_index as usize).unwrap();
                    temp_request.add_time_in_queue();
                    queue_size -= 1;
                }
            }

            if queue_basic.size() > 0 && queue_rt.size() == 0 {
                let first_in_queue = queue_basic.list[0] as usize;
                let processed_request = requests.get(first_in_queue).unwrap();
                let current_block = processed_request.get_block_num();
                if current_block > head_position {
                    head_position += 1;
                    head_moves += 1;
                } else if current_block < head_position {
                    head_position -= 1;
                    head_moves += 1;
                } else {
                    queue_basic.remove(0);
                }
            }
        }

        if queue_basic.is_empty() && queue_rt.is_empty() && requests.len() == (request_no as usize)
        {
            break;
        }
    }

    (head_moves, realtime_handled, realtime_starved)
}

fn fdscan(requests: &mut Vec<Request>) -> (i32, i32, i32) {
    let mut disk_array: [i32; BLOCK_SIZE as usize] = [0; BLOCK_SIZE as usize];
    let mut queue_rt = Queue::new(vec![]);
    let mut head_moves = 0;
    let mut head_position = 0;
    let mut rng = rand::thread_rng();
    let mut request_no = 0;
    let mut realtime_handled = 0;
    let mut realtime_starved = 0;
    let mut active_requests = 0;

    loop {
        if rng.gen_range(1, RNG_MAX) >= RNG_CHANCE && request_no < requests.len() as i32 {
            let current_request = requests.get(request_no as usize).unwrap();
            let temp_index = current_request.get_block_num();
            if current_request.is_realtime() {
                queue_rt.push_request(request_no);
                queue_rt.rt_insertion_sort(requests);
                disk_array[temp_index as usize] += 1;
                active_requests += 1;
            } else {
                disk_array[temp_index as usize] += 1;
                active_requests += 1;
            }
            request_no += 1;
        }
        loop {
            if queue_rt.size() > 0 {
                let first_in_queue = requests.get(queue_rt.list[0] as usize).unwrap();
                if !first_in_queue.is_reachable(head_position) {
                    queue_rt.remove(0);
                    realtime_starved += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        if active_requests > 0 {
            active_requests -= disk_array[head_position as usize];
            disk_array[head_position as usize] = 0;
            if queue_rt.size() > 0 {
                let current_block = requests
                    .get(queue_rt.list[0] as usize)
                    .unwrap()
                    .get_block_num();
                if current_block > head_position {
                    head_position += 1;
                    head_moves += 1;
                } else if current_block < head_position {
                    head_position -= 1;
                    head_moves += 1;
                } else {
                    realtime_handled += queue_rt.remove_at_pos(head_position, requests);
                    queue_rt.rt_insertion_sort(requests);
                }
                let mut queue_size = queue_rt.size() as i32;
                let mut i = 0;
                while queue_size > i {
                    let temp_index = queue_rt.list[i as usize];
                    let temp_request = requests.get(temp_index as usize).unwrap();
                    if temp_request.time_remaining() == 0 {
                        queue_rt.remove(i as usize);
                        queue_size -= 1;
                        realtime_starved += 1;
                        continue;
                    }
                    i += 1;
                }

                let mut queue_size = queue_rt.size() as i32;
                while queue_size > 0 {
                    let temp_index = queue_rt.list[(queue_size - 1) as usize];
                    let temp_request = requests.get_mut(temp_index as usize).unwrap();
                    temp_request.add_time_in_queue();
                    queue_size -= 1;
                }
                realtime_handled += queue_rt.remove_at_pos(head_position, requests);
            } else {
                head_position += 1;
                head_moves += 1;

                if head_position > BLOCK_SIZE - 1 {
                    head_position = 0;
                }

                if disk_array[head_position as usize] != 0 {
                    active_requests -= disk_array[head_position as usize];
                    disk_array[head_position as usize] = 0;
                }
            }
        } else if active_requests == 0 && request_no == (requests.len() as i32) {
            break;
        }
    }

    (head_moves, realtime_handled, realtime_starved)
}
