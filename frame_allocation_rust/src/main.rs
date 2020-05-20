mod process;

use process::Process;
use rand::prelude::*;

const FRAMES_NO: i32 = 30;
const PROCESSES_NO: i32 = 10;

fn main() {
    let frames: [i32; FRAMES_NO as usize] = [0; FRAMES_NO as usize];
    let mut processes: Vec<Process> = generate_processes();
    generate_requests(&mut processes);
    let requests_no = get_requests_no(&mut processes);
    let queue = generate_global_queue(&mut processes, requests_no);
    equal_allocation(&mut processes);
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

fn generate_global_queue(processes: &mut Vec<Process>, requests_no: i32) -> Vec<i32> {
    let mut queue: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();
    let mut current_processes: [i32; PROCESSES_NO as usize] = [0; PROCESSES_NO as usize];
    let mut counter = 0;
    while counter < requests_no {
        let temp = rng.gen_range(0, PROCESSES_NO);
        let temp_proc = processes.get(temp as usize).unwrap();
        if current_processes[temp as usize] != temp_proc.requests.len() as i32 {
            queue.push(
                *temp_proc
                    .requests
                    .get(current_processes[temp as usize] as usize)
                    .unwrap(),
            );
            current_processes[temp as usize] += 1;
        } else {
            continue;
        }
        counter += 1;
    }
    queue
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
