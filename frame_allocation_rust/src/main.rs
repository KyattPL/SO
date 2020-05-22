mod process;

use process::Process;
use rand::prelude::*;

const FRAMES_NO: i32 = 30;
const PROCESSES_NO: i32 = 10;
const UPPER_PFF: f32 = 0.6;
const LOWER_PFF: f32 = 0.3;

fn main() {
    let mut processes: Vec<Process> = generate_processes();
    generate_requests(&mut processes);
    //proportional_allocation(&mut processes);
    let requests_no = get_requests_no(&mut processes);
    equal_allocation(&mut processes);
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
    let mut free_frames: Vec<i32> = Vec::new();

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

    let mut debugger = 0;

    while counter < requests_no {
        debugger += 1;
        let temp = rng.gen_range(0, PROCESSES_NO);
        if debugger % 1000 == 0 {
            print_motherfucking_garbo(processes, &free_frames);
        }

        let temp_proc = processes.get_mut(temp as usize).unwrap();

        if temp_proc.is_stopped {
            if free_frames.is_empty() {
                continue;
            } else {
                add_new_frame(temp_proc, &mut free_frames);
            }
        }

        temp_proc.time_window += 1;
        if temp_proc.requests.len() as i32 != current_requests[temp as usize] {
            if !frames.contains(
                temp_proc
                    .requests
                    .get(current_requests[temp as usize] as usize)
                    .unwrap(),
            ) {
                let mut frame_counter = 0;
                let mut min = requests_no;
                let mut index = temp_proc.frames[frame_counter];
                let mut min_index = temp_proc.frames[temp_proc.frames.len() - 1];
                let mut scan_index = current_requests[temp as usize];

                while index != temp_proc.frames[temp_proc.frames.len() - 1] {
                    if frames[index as usize]
                        == *temp_proc.requests.get(scan_index as usize).unwrap()
                    {
                        if min > scan_index {
                            min = scan_index;
                            min_index = index;
                        }
                        frame_counter += 1;
                        index = temp_proc.frames[frame_counter];
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
                temp_proc.page_faults += 1;
            } else {
                current_requests[temp as usize] += 1;
            }
        }
        if temp_proc.time_window == 20 {
            page_fault_frequency(temp_proc, &mut free_frames);
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
    let mut iterator = 0;
    while iterator < processes.len() {
        processes[iterator].set_first_frame(current_frame);
        current_frame += coeff;
        if modulo != 0 {
            processes[iterator].set_last_frame(current_frame);
            current_frame += 1;
            modulo -= 1;
        } else {
            processes[iterator].set_last_frame(current_frame - 1);
        }
        iterator += 1;
    }
    iterator = 0;
    let mut current_process = 0;
    while current_process < PROCESSES_NO {
        let temp = processes[current_process as usize].get_last_frame();
        while iterator <= temp as usize {
            processes[current_process as usize]
                .frames
                .push(iterator as i32);
            iterator += 1;
        }
        current_process += 1;
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
        current_frame += coeffs[iterator].round() as i32;
        processes[iterator as usize].set_last_frame(current_frame - 1);
        iterator += 1;
    }

    iterator = 0;
    let mut current_process = 0;
    while current_process < PROCESSES_NO {
        let temp = processes[current_process as usize].get_last_frame();
        while iterator <= temp as usize {
            processes[current_process as usize]
                .frames
                .push(iterator as i32);
            iterator += 1;
        }
        current_process += 1;
    }
}

fn page_fault_frequency(process: &mut Process, free_frames: &mut Vec<i32>) {
    let pff = process.calculate_pff();
    process.time_window = 0;
    process.page_faults = 0;

    if pff < LOWER_PFF && process.frames.len() > 1 {
        free_frames.push(process.frames.pop().unwrap());
    } else if pff > UPPER_PFF {
        if !free_frames.is_empty() {
            process.frames.push(free_frames.pop().unwrap());
        } else {
            process.is_stopped = true;
            stop_process(process, free_frames);
        }
    }
}

fn add_new_frame(process: &mut Process, free_frames: &mut Vec<i32>) {
    process.frames.push(free_frames.pop().unwrap());
    process.is_stopped = false;
}

fn stop_process(process: &mut Process, free_frames: &mut Vec<i32>) {
    let mut iterator = 0;
    while iterator < process.frames.len() {
        free_frames.push(process.frames.pop().unwrap());
        iterator += 1;
    }
}

fn print_motherfucking_garbo(processes: &Vec<Process>, free_frames: &Vec<i32>) {
    let mut iterator = 0;
    while iterator < processes.len() {
        let mut inner_it = 0;
        print!("Proc {}: ", iterator);
        while inner_it < processes[iterator].frames.len() {
            print!("{} ", processes[iterator].frames[inner_it]);
            inner_it += 1;
        }
        println!();
        iterator += 1;
    }
    iterator = 0;
    println!("Free frames: ");
    while iterator < free_frames.len() {
        print!("{} ", free_frames[iterator]);
        iterator += 1;
    }
}
