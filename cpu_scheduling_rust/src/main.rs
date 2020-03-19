mod process;
mod queue;

use process::Process;
use queue::Queue;
use rand::prelude::*;

fn main() {
    let mut v = generate_processes(100000);

    let iterations = fcfs(&mut v);
    println!("Number of iterations: {}", iterations);
    println!("Average time in queue: {}", average_time_in_queue(&mut v));
}

fn generate_processes(mut num_of_processes: i32) -> Vec<Process> {
    let mut processes: Vec<Process> = vec![];
    let mut rng = thread_rng();
    while num_of_processes != 0 {
        let temp_time = rng.gen_range(5, 50);
        processes.push(Process::new(temp_time));
        num_of_processes -= 1;
    }
    processes
}

fn average_time_in_queue(processes: &mut Vec<Process>) -> f64 {
    let mut size = processes.len() - 1;
    let mut sum = 0;
    while size != 0 {
        sum += processes.get(size).unwrap().get_time_in_queue();
        size -= 1;
    }
    sum as f64 / processes.len() as f64
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
