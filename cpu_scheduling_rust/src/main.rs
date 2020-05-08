mod process;
mod queue;

use process::Process;
use queue::Queue;
use rand::prelude::*;
use std::fs;

fn main() {
    let path = String::from("data_rand.txt");
    //let mut v = generate_processes(100000);
    //generate_nonrandom_input(&path);

    let mut v = read_from_file(&path);
    let iterations = fcfs(&mut v);
    let mut str1: String = String::from("FCFS:\n");
    str1.push_str(&format!("Number of iterations: {}\n", iterations));
    str1.push_str(&format!(
        "Average time in queue: {}\n",
        average_time_in_queue(&v)
    ));
    str1.push_str(&format!("Max time in queue: {}\n\n", max_time_in_queue(&v)));

    print!("{}", str1);

    let mut v = read_from_file(&path);
    let (iterations, number_of_jumps) = rr(&mut v);
    let mut str2: String = String::from("Round Robin:\n");
    str2.push_str(&format!("Number of iterations: {}\n", iterations));
    str2.push_str(&format!("Number of jumps: {}\n\n", number_of_jumps));

    print!("{}", str2);

    let mut v = read_from_file(&path);
    let (iterations, starving_processes) = sjf_preemptive(&mut v);
    let mut str3: String = String::from("SFJ-preemptive:\n");
    str3.push_str(&format!("Number of iterations: {}\n", iterations));
    str3.push_str(&format!(
        "Average time in queue: {}\n",
        average_time_in_queue(&v)
    ));
    str3.push_str(&format!(
        "Processes that have been starving: {}",
        starving_processes
    ));

    println!("{}", str3);
    save_results(format!("{}{}{}", str1, str2, str3));
}

#[allow(dead_code)]
fn generate_processes(mut num_of_processes: i32) -> Vec<Process> {
    let mut processes: Vec<Process> = vec![];
    let mut rng = thread_rng();
    let mut string = String::from("");
    while num_of_processes != 0 {
        let temp_time = rng.gen_range(5, 50);
        string.push_str(&temp_time.to_string());
        string.push_str("\n");
        processes.push(Process::new(temp_time));
        num_of_processes -= 1;
    }
    write_to_file(string, &String::from("data_rand.txt"));
    processes
}

#[allow(dead_code)]
fn generate_nonrandom_input(path: &str) {
    let mut counter = 1;
    let mut string = String::from("");
    while counter <= 200 {
        string.push_str(&counter.to_string());
        string.push_str("\n");
        counter += 1;
    }
    write_to_file(string, path);
}

#[allow(dead_code)]
fn write_to_file(data: String, path: &str) {
    fs::write(path, data).expect("Unable to write to the file");
}

fn read_from_file(path: &str) -> Vec<Process> {
    let data = fs::read_to_string(path).expect("Unable to read the file");
    let mut processes: Vec<Process> = Vec::new();
    for line in data.lines() {
        processes.push(Process::new(line.parse().unwrap()));
    }
    processes
}

fn save_results(data: String) {
    fs::write("results.txt", data).expect("Unable to write to the file");
}

fn average_time_in_queue(processes: &[Process]) -> f64 {
    let mut size = processes.len() - 1;
    let mut sum = 0;
    while size != 0 {
        sum += processes.get(size).unwrap().get_time_in_queue();
        size -= 1;
    }
    sum as f64 / processes.len() as f64
}

fn max_time_in_queue(processes: &[Process]) -> i32 {
    let mut max = 0;
    let mut size = processes.len() - 1;
    while size != 0 {
        let time = processes.get(size).unwrap().get_time_in_queue();
        if time > max {
            max = time;
        }
        size -= 1;
    }
    max
}

fn fcfs(processes: &mut Vec<Process>) -> i32 {
    let mut queue = Queue::new(vec![0]);

    let mut rng = rand::thread_rng();
    let mut iterations: i32 = 0;
    let mut process_no: i32 = 1;

    loop {
        if rng.gen_range(0, 100) >= 97 && process_no < processes.len() as i32 {
            queue.push_process(process_no);
            process_no += 1;
            if process_no % 100 == 0 {
                //println!("Process no: {}", process_no);
            }
        }

        if queue.size() > 0 {
            let first_in_queue: usize = queue.list[0] as usize;
            let processed_task = processes.get_mut(first_in_queue).unwrap();
            processed_task.add_time_processed();
            if processed_task.get_time_processed() == processed_task.get_task_time() {
                queue.remove(0);
            }

            let mut size_of_queue: i32 = queue.size() as i32;
            while size_of_queue != 1 && size_of_queue != 0 {
                let queue_no: usize = queue.list[(size_of_queue - 1) as usize] as usize;
                processes.get_mut(queue_no).unwrap().add_time_in_queue();
                size_of_queue -= 1;
            }

            if (processes.len() as i32) == process_no && queue.is_empty() {
                break;
            }

            iterations += 1;
        }
    }
    iterations
}

fn rr(processes: &mut Vec<Process>) -> (i32, i32) {
    let mut queue = Queue::new(vec![0]);

    let mut rng = rand::thread_rng();
    let mut iterations: i32 = 0;
    let mut process_no: i32 = 1;
    let mut number_of_jumps: i32 = 0;
    let quant = 1;
    let mut quant_counter = 0;
    let mut index = 0;

    loop {
        if rng.gen_range(0, 100) >= 99 && process_no < processes.len() as i32 {
            queue.push_process(process_no);
            process_no += 1;
        }

        if (processes.len() as i32) == process_no && queue.is_empty() {
            break;
        }

        if queue.size() > 0 {
            iterations += 1;
            let process = processes.get_mut(queue.list[index] as usize).unwrap();
            process.add_time_processed();
            if process.get_time_processed() == process.get_task_time() {
                queue.remove(index as usize);
                quant_counter = 0;
                if index == queue.size() {
                    index = 0;
                }
                continue;
            }

            if quant_counter == quant - 1 {
                index += 1;
                quant_counter = 0;
                number_of_jumps += 1;
                if index == queue.size() {
                    index = 0;
                }
                continue;
            }
        }
    }
    (iterations, number_of_jumps)
}

fn sjf_preemptive(processes: &mut Vec<Process>) -> (i32, i32) {
    let mut queue = Queue::new(vec![0]);
    let mut rng = rand::thread_rng();
    let mut process_no = 1;
    let mut iterations = 0;
    let mut starving_processes = 0;
    loop {
        if rng.gen_range(0, 100) >= 97 && process_no < processes.len() as i32 {
            queue.push_process(process_no);
            queue.insertion_sort(processes);
            //println!("{:?}", queue);
            process_no += 1;
        }

        if queue.size() > 0 {
            let first_in_queue: usize = queue.list[0] as usize;
            let processed_task = processes.get_mut(first_in_queue).unwrap();
            processed_task.add_time_processed();
            if processed_task.get_time_processed() == processed_task.get_task_time() {
                if processed_task.get_task_time() >= 5 * processed_task.get_time_in_queue() {
                    starving_processes += 1;
                }
                queue.remove(0);
            }

            let mut size_of_queue: i32 = queue.size() as i32;
            while size_of_queue != 1 && size_of_queue != 0 {
                let queue_no: usize = queue.list[(size_of_queue - 1) as usize] as usize;
                processes.get_mut(queue_no).unwrap().add_time_in_queue();
                size_of_queue -= 1;
            }

            if (processes.len() as i32) == process_no && queue.is_empty() {
                break;
            }
            iterations += 1;
        }
    }
    (iterations, starving_processes)
}
