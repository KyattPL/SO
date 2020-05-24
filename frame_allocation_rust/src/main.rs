mod process;

use process::Process;
use rand::prelude::*;
use std::collections::HashSet;

const FRAMES_NO: i32 = 30;
const PROCESSES_NO: i32 = 10;
const UPPER_PFF: f32 = 0.6;
const LOWER_PFF: f32 = 0.3;

fn main() {
    let mut processes: Vec<Process> = generate_processes();
    generate_requests(&mut processes);
    let requests_no = get_requests_no(&mut processes);
    let queue = generate_queue(requests_no);

    equal_allocation(&mut processes);
    let page_faults = lru(&mut processes, requests_no, &queue, 0);
    println!("Requests no: {}", requests_no);
    print_page_faults(page_faults, &mut processes);

    proportional_allocation(&mut processes);
    let page_faults = lru(&mut processes, requests_no, &queue, 1);
    println!("Requests no: {}", requests_no);
    print_page_faults(page_faults, &mut processes);

    proportional_allocation(&mut processes);
    let page_faults = lru(&mut processes, requests_no, &queue, 2);
    println!("Requests no: {}", requests_no);
    print_page_faults(page_faults, &mut processes);

    proportional_allocation(&mut processes);
    let page_faults = lru(&mut processes, requests_no, &queue, 3);
    println!("Requests no: {}", requests_no);
    print_page_faults(page_faults, &mut processes);
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

fn generate_queue(requests_no: i32) -> Vec<i32> {
    let mut queue: Vec<i32> = Vec::new();
    let mut iterator = 0;
    let mut rng = rand::thread_rng();

    while iterator < requests_no {
        let temp = rng.gen_range(0, PROCESSES_NO);
        queue.push(temp);
        iterator += 1;
    }
    queue
}

fn get_requests_no(processes: &mut Vec<Process>) -> i32 {
    let mut requests_sum = 0;
    for proc in processes {
        requests_sum += proc.requests.len() as i32;
    }
    requests_sum
}

fn print_page_faults(page_faults: [i32; PROCESSES_NO as usize], processes: &mut Vec<Process>) {
    let mut iterator = 0;
    let mut sum = 0;
    let mut sum_stops = 0;
    let mut sum_thrashing = 0;
    while iterator < page_faults.len() {
        println!(
            "Process no: {}, nof page faults: {}, nof stops: {}, nof thrashing: {}",
            iterator,
            page_faults[iterator],
            processes[iterator].how_many_stops,
            processes[iterator].how_much_thrashing
        );
        sum += page_faults[iterator];
        sum_stops += processes[iterator].how_many_stops;
        sum_thrashing += processes[iterator].how_much_thrashing;
        iterator += 1;
    }
    println!("Sum of all page faults: {}", sum);
    println!(
        "Avg page faults per process: {}",
        (sum as f32) / (PROCESSES_NO as f32)
    );
    println!("Sum of all process stops: {}", sum_stops);
    println!(
        "Avg stops per process: {}",
        (sum_stops as f32) / (PROCESSES_NO as f32)
    );
    println!("Sum of all process thrashing: {}", sum_thrashing);
    println!(
        "Avg thrashing per process: {}",
        (sum_thrashing as f32) / (PROCESSES_NO as f32)
    );
    println!();
}

fn lru(
    processes: &mut Vec<Process>,
    requests_no: i32,
    queue: &[i32],
    flag: i32,
) -> [i32; PROCESSES_NO as usize] {
    let mut frames: [i32; FRAMES_NO as usize] = [0; FRAMES_NO as usize];
    let mut initializer = 0;
    let mut rng = rand::thread_rng();
    let mut current_requests: [i32; PROCESSES_NO as usize] = [0; PROCESSES_NO as usize];
    let mut current_process = 0;
    let mut counter = 0;
    let mut page_faults: [i32; PROCESSES_NO as usize] = [0; PROCESSES_NO as usize];
    let mut free_frames: Vec<i32> = Vec::new();
    let mut time_window = 0;

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
        let mut temp = queue[counter as usize];
        let mut temp_proc = processes.get_mut(temp as usize).unwrap();

        while temp_proc.is_stopped {
            if free_frames.len() >= temp_proc.frames_wanted as usize {
                add_new_frames(temp_proc, &mut free_frames);
            } else {
                temp = rng.gen_range(0, PROCESSES_NO);
                temp_proc = processes.get_mut(temp as usize).unwrap();
            }
        }

        time_window += 1;
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

                if flag == 3 {
                    temp_proc
                        .unique_pages
                        .insert(*temp_proc.requests.get(scan_index as usize).unwrap());
                }

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
        if time_window == 40 {
            if flag == 2 {
                page_fault_frequency(temp_proc, &mut free_frames);
            }
            if temp_proc.page_faults >= 20 {
                temp_proc.how_much_thrashing += 1;
            }
            temp_proc.page_faults = 0;
            if flag == 3 {
                calculate_d(processes, &mut free_frames);
            }
            time_window = 0;
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
            if coeffs[temp as usize].round() as i32 != 1 {
                coeffs[temp as usize] -= 1.0;
            } else {
                continue;
            }
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

    if pff < LOWER_PFF && process.frames.len() > 1 {
        free_frames.push(process.frames.pop().unwrap());
    } else if pff > UPPER_PFF {
        if !free_frames.is_empty() {
            process.frames.push(free_frames.pop().unwrap());
        } else {
            process.frames_wanted = process.frames.len() as i32 + 1;
            stop_process(process, free_frames);
        }
    }
}

fn add_new_frames(process: &mut Process, free_frames: &mut Vec<i32>) {
    let mut iterator = 0;
    while iterator < process.frames_wanted {
        process.frames.push(free_frames.pop().unwrap());
        iterator += 1;
    }
    process.is_stopped = false;
    process.frames_wanted = 0;
}

fn stop_process(process: &mut Process, free_frames: &mut Vec<i32>) {
    let mut iterator = 0;
    process.how_many_stops += 1;
    process.is_stopped = true;
    let frames_no = process.frames.len();
    while iterator < frames_no {
        free_frames.push(process.frames.pop().unwrap());
        iterator += 1;
    }
}

fn calculate_d(processes: &mut Vec<Process>, free_frames: &mut Vec<i32>) {
    let mut iterator = 0;
    let mut d = 0;
    while iterator < processes.len() {
        d += processes[iterator].calculate_wss();
        iterator += 1;
    }
    if d <= FRAMES_NO {
        wss_allocation(processes, free_frames);
    } else {
        while d > FRAMES_NO {
            let index = lowest_wss_proc(processes);
            d -= processes[index].frames.len() as i32;
            processes[index].frames_wanted = processes[index].calculate_wss();
            stop_process(&mut processes[index], free_frames);
        }
        proportional_wss(processes, free_frames);
    }
    iterator = 0;
    while iterator < processes.len() {
        processes[iterator].unique_pages = HashSet::new();
        iterator += 1;
    }
}

fn wss_allocation(processes: &mut Vec<Process>, free_frames: &mut Vec<i32>) {
    let mut iterator = 0;
    while iterator < processes.len() {
        let mut frames_no = processes[iterator].frames.len();
        while frames_no != 0 {
            free_frames.push(processes[iterator].frames.pop().unwrap());
            frames_no -= 1;
        }
        iterator += 1;
    }

    iterator = 0;
    while iterator < processes.len() {
        let mut wss = processes[iterator].calculate_wss();
        if wss == 0 {
            processes[iterator].frames_wanted = 1;
            stop_process(&mut processes[iterator], free_frames);
        }
        while wss != 0 {
            processes[iterator].frames.push(free_frames.pop().unwrap());
            wss -= 1;
        }
        iterator += 1;
    }
}

fn lowest_wss_proc(processes: &mut Vec<Process>) -> usize {
    let mut lowest_index = 0;
    let mut min = FRAMES_NO;
    let mut iterator = 0;
    while iterator < processes.len() {
        if processes[iterator].calculate_wss() < min && !processes[iterator].is_stopped {
            min = processes[iterator].calculate_wss();
            lowest_index = iterator;
        }
        iterator += 1;
    }
    lowest_index
}

fn proportional_wss(processes: &mut Vec<Process>, free_frames: &mut Vec<i32>) {
    let mut iterator = 0;
    let mut working_processes = 0;
    let mut sum_all_wss = 0;
    let mut coeffs: Vec<f64> = Vec::new();
    let mut sum_all_frames = 0;
    let mut rng = rand::thread_rng();
    let mut current_frame = 0;

    while iterator < processes.len() {
        if !processes[iterator].is_stopped {
            working_processes += 1;
            sum_all_wss += processes[iterator].calculate_wss();
        }
        iterator += 1;
    }

    iterator = 0;
    while iterator < processes.len() {
        if !processes[iterator].is_stopped {
            let coeff = (processes[iterator].calculate_wss() as f64) / (sum_all_wss as f64)
                * (FRAMES_NO as f64);
            coeffs.push(coeff);
            processes[iterator].frames.clear();
            sum_all_frames += coeff.round() as i32;
        }
        iterator += 1;
    }

    while sum_all_frames != FRAMES_NO {
        let temp = rng.gen_range(0, working_processes);
        if sum_all_frames > FRAMES_NO {
            if coeffs[temp as usize].round() as i32 != 1 {
                coeffs[temp as usize] -= 1.0;
            } else {
                continue;
            }
        } else {
            coeffs[temp as usize] += 1.0;
        }
        sum_all_frames = sum_of_array(&coeffs);
    }

    iterator = 0;
    let mut coeff_iter = 0;
    while iterator < processes.len() {
        if !processes[iterator].is_stopped {
            processes[iterator as usize].set_first_frame(current_frame);
            current_frame += coeffs[coeff_iter].round() as i32;
            processes[iterator as usize].set_last_frame(current_frame - 1);
            coeff_iter += 1;
        }
        iterator += 1;
    }

    iterator = 0;
    let mut current_process = 0;
    while current_process < PROCESSES_NO {
        if !processes[current_process as usize].is_stopped {
            let temp = processes[current_process as usize].get_last_frame();
            while iterator <= temp as usize {
                processes[current_process as usize]
                    .frames
                    .push(iterator as i32);
                iterator += 1;
            }
        }
        current_process += 1;
    }
    *free_frames = Vec::new();
}
