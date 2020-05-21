mod process;

use process::Process;
use rand::prelude::*;

const FRAMES_NO: i32 = 30;
const PROCESSES_NO: i32 = 10;

fn main() {
    let mut processes: Vec<Process> = generate_processes();
    generate_requests(&mut processes);
    proportional_allocation(&mut processes);
    let requests_no = get_requests_no(&mut processes);
    //equal_allocation(&mut processes);
    let page_faults = lru(&mut processes, requests_no);
    println!("Requests no: {}", requests_no);
    print_page_faults(page_faults);
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

fn print_page_faults(page_faults: [i32; PROCESSES_NO as usize]) {
    let mut iterator = 1;
    let mut sum = 0;
    while iterator <= page_faults.len() {
        println!(
            "Process no: {}, no page faults: {}",
            iterator,
            page_faults[iterator - 1]
        );
        sum += page_faults[iterator - 1];
        iterator += 1;
    }
    println!("Sum of all page faults: {}", sum);
    println!(
        "Avg page faults per process: {}",
        (sum as f32) / (PROCESSES_NO as f32)
    );
}

fn lru(processes: &mut Vec<Process>, requests_no: i32) -> [i32; PROCESSES_NO as usize] {
    let mut frames: [i32; FRAMES_NO as usize] = [0; FRAMES_NO as usize];
    let mut initializer = 0;
    let mut rng = rand::thread_rng();
    let mut current_requests: [i32; PROCESSES_NO as usize] = [0; PROCESSES_NO as usize];
    let mut current_process = 0;
    let mut counter = 0;
    let mut page_faults: [i32; PROCESSES_NO as usize] = [0; PROCESSES_NO as usize];

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
                page_faults[temp as usize] += 1;
            } else {
                current_requests[temp as usize] += 1;
            }
        }
        counter += 1;
    }
    page_faults
}

#[allow(dead_code)]
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

fn sum_of_array(arr: &[f64]) -> i32 {
    let mut iter = 0;
    let mut sum = 0;
    while iter < arr.len() {
        sum += arr[iter as usize].round() as i32;
        iter += 1;
    }
    sum
}

#[allow(dead_code)]
fn proportional_allocation(processes: &mut Vec<Process>) {
    let mut rng = rand::thread_rng();
    let mut iterator = 0;
    let mut sum_all_pages = 0;
    let mut current_frame = 0;
    while iterator < processes.len() {
        sum_all_pages += processes[iterator].pages_no();
        iterator += 1;
    }
    let mut coeffs: Vec<f64> = Vec::new();
    iterator = 0;
    let mut sum_all_frames = 0;
    while iterator < processes.len() {
        let coeff =
            (processes[iterator].pages_no() as f64) / (sum_all_pages as f64) * (FRAMES_NO as f64);
        coeffs.push(coeff);
        sum_all_frames += coeff.round() as i32;
        iterator += 1;
    }

    while sum_all_frames != FRAMES_NO {
        let temp = rng.gen_range(0, PROCESSES_NO);
        if sum_all_frames > FRAMES_NO {
            coeffs[temp as usize] -= 1.0;
        } else {
            coeffs[temp as usize] += 1.0;
        }
        sum_all_frames = sum_of_array(&coeffs);
    }

    iterator = 0;
    while iterator < processes.len() {
        processes[iterator as usize].set_first_frame(current_frame);
        current_frame = coeffs[iterator].round() as i32;
        processes[iterator as usize].set_last_frame(current_frame - 1);
        iterator += 1;
    }
}
