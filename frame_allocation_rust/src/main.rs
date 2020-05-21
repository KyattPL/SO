mod process;

use process::Process;
use rand::prelude::*;

const FRAMES_NO: i32 = 30;
const PROCESSES_NO: i32 = 10;

fn main() {
    let mut processes: Vec<Process> = generate_processes();
    generate_requests(&mut processes);
    let requests_no = get_requests_no(&mut processes);
    equal_allocation(&mut processes);
    let page_faults = lru(&mut processes, requests_no);
    println!("requests no: {}", requests_no);
    println!("page faults: {}", page_faults);
}

fn generate_processes() -> Vec<Process> {
    let mut i = 0;
    let mut rng = rand::thread_rng();
    let mut processes = Vec::new();
    let mut current_max = 20;
    let mut current_min = 0;
    while i < PROCESSES_NO {
        let temp = rng.gen_range(current_min + 1, current_max);
        processes.push(Process::new(current_min, temp));
        current_min = temp;
        current_max = temp + 19;
        i += 1;
    }
    processes
}

fn generate_requests(processes: &mut Vec<Process>) {
    for proc in processes {
        proc.generate_requests();
    }
}

fn get_requests_no(processes: &mut Vec<Process>) -> i32 {
    let mut requests_sum = 0;
    for proc in processes {
        requests_sum += proc.requests.len() as i32;
    }
    requests_sum
}

fn lru(processes: &mut Vec<Process>, requests_no: i32) -> i32 {
    let mut frames: [i32; FRAMES_NO as usize] = [0; FRAMES_NO as usize];
    let mut initializer = 0;
    let mut rng = rand::thread_rng();
    let mut current_requests: [i32; PROCESSES_NO as usize] = [0; PROCESSES_NO as usize];
    let mut current_process = 0;
    let mut counter = 0;
    let mut page_faults = 0;

    while initializer < FRAMES_NO {
        let temp = processes.get(current_process as usize).unwrap();
        if current_requests[current_process as usize] != temp.frames_no() {
            frames[initializer as usize] =
                temp.requests[current_requests[current_process as usize] as usize];
            current_requests[current_process as usize] += 1;
        } else {
            counter += current_requests[current_process as usize];
            current_process += 1;
            continue;
        }
        initializer += 1;
    }

    while counter < requests_no {
        let temp = rng.gen_range(0, PROCESSES_NO);
        let temp_proc = processes.get(temp as usize).unwrap();
        if temp_proc.requests.len() as i32 != current_requests[temp as usize] {
            if !frames.contains(
                temp_proc
                    .requests
                    .get(current_requests[temp as usize] as usize)
                    .unwrap(),
            ) {
                let mut min = requests_no;
                let mut index = temp_proc.first_frame;
                let mut min_index = temp_proc.first_frame;
                let mut scan_index = current_requests[temp as usize];
                while index < temp_proc.last_frame {
                    if frames[index as usize]
                        == *temp_proc.requests.get(scan_index as usize).unwrap()
                    {
                        if min > scan_index {
                            min = scan_index;
                            min_index = index;
                        }
                        index += 1;
                        scan_index = current_requests[temp as usize];
                        continue;
                    }
                    scan_index -= 1;
                    if scan_index == -1 {
                        min_index = index;
                        break;
                    }
                }
                frames[min_index as usize] = *temp_proc
                    .requests
                    .get(current_requests[temp as usize] as usize)
                    .unwrap();
                current_requests[temp as usize] += 1;
                page_faults += 1;
            } else {
                current_requests[temp as usize] += 1;
            }
        }
        counter += 1;
    }
    println!("{}", counter);
    page_faults
}

fn equal_allocation(processes: &mut Vec<Process>) {
    let coeff: i32 = FRAMES_NO / PROCESSES_NO;
    let mut current_frame = 0;
    let mut modulo = FRAMES_NO % PROCESSES_NO;
    for proc in processes {
        proc.set_first_frame(current_frame);
        current_frame += coeff;
        if modulo != 0 {
            proc.set_last_frame(current_frame);
            current_frame += 1;
            modulo -= 1;
        } else {
            proc.set_last_frame(current_frame - 1);
        }
    }
}
