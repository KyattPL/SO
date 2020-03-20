mod process;
mod queue;

use process::Process;
use queue::Queue;
use rand::prelude::*;
use std::fs;

fn main() {
    //let mut v = generate_processes(100000);
    let mut v = read_from_file();

    let iterations = fcfs(&mut v);
    println!("Number of iterations: {}", iterations);
    println!("Average time in queue: {}", average_time_in_queue(&v));
    println!("Max time in queue: {}", max_time_in_queue(&v));

    println!();

    let mut v = read_from_file();
    let (iterations, number_of_jumps) = rr(&mut v);
    println!("Number of iterations: {}", iterations);
    println!("Number of jumps: {}", number_of_jumps);

    println!();

    let mut v = read_from_file();
    let (iterations, starving_processes) = sjf_preemptive(&mut v);
    println!("Number of iterations: {}", iterations);
    println!("Average time in queue: {}", average_time_in_queue(&v));
    println!("Processes that have been starving: {}", starving_processes);
}

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
    write_to_file(string);
    processes
}

fn write_to_file(data: String) {
    fs::write("data.txt", data).expect("Unable to write to the file");
}

fn read_from_file() -> Vec<Process> {
    let data = fs::read_to_string("data.txt").expect("Unable to read the file");
    let mut processes: Vec<Process> = Vec::new();
    for line in data.lines() {
        processes.push(Process::new(line.parse().unwrap()));
    }
    processes
}

fn average_time_in_queue(processes: &Vec<Process>) -> f64 {
    let mut size = processes.len() - 1;
    let mut sum = 0;
    while size != 0 {
        sum += processes.get(size).unwrap().get_time_in_queue();
        size -= 1;
    }
    sum as f64 / processes.len() as f64
}

fn max_time_in_queue(processes: &Vec<Process>) -> i32 {
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

fn sjf_preemptive(processes: &mut Vec<Process>) -> (i32,i32) {
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
                if processed_task.get_task_time() >= 5*processed_task.get_time_in_queue() {
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