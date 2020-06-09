mod cpu;
mod process;

use cpu::CPU;
use process::Process;
use rand::*;
use std::collections::HashSet;

const P: i32 = 70;
const R: i32 = 30;
const Z: i32 = 10;
const NO_CPUS: i32 = 50;
const NO_PROCESSES: i32 = 100_000;
const RAND_PROCESS_CHANCE: i32 = 85;

fn main() {
    let mut cpus = setup_cpus(NO_CPUS);
    let (fails, time_passed) = dispenser(&mut cpus);
    println!("No failed processes: {}", fails);
    let global_avg = avg_load(&mut cpus, time_passed);
    standard_deviation(&mut cpus, global_avg, time_passed);

    println!();
    clear_cpus(&mut cpus);

    let (fails, time_passed) = fair_work(&mut cpus);
    println!("No failed processes: {}", fails);
    let global_avg = avg_load(&mut cpus, time_passed);
    standard_deviation(&mut cpus, global_avg, time_passed);

    println!();
    clear_cpus(&mut cpus);

    let (fails, time_passed) = hardworking(&mut cpus);
    println!("No failed processes: {}", fails);
    let global_avg = avg_load(&mut cpus, time_passed);
    standard_deviation(&mut cpus, global_avg, time_passed);
}

fn avg_load(cpus: &mut Vec<CPU>, time: i32) -> f32 {
    let mut iter = 0;
    let mut global_avg = 0.0;
    while iter < NO_CPUS {
        let temp_cpu = cpus.get(iter as usize).unwrap();
        let avg = (temp_cpu.get_sum_of_load() as f32) / (time as f32);
        println!("CPU no {}, avg load: {}", iter + 1, avg);
        iter += 1;
        global_avg += avg;
    }
    (global_avg as f32) / (NO_CPUS as f32)
}

fn standard_deviation(cpus: &mut Vec<CPU>, global_avg: f32, time: i32) {
    let mut iter = 0;
    let mut sum = 0.0;
    while iter < NO_CPUS {
        let temp = (cpus.get(iter as usize).unwrap().get_sum_of_load() as f32) / (time as f32);
        let temp = temp - global_avg;
        sum += temp * temp;
        iter += 1;
    }
    sum = sum / (NO_CPUS - 1) as f32;
    println!("Standard deviation = {}", sum.sqrt());
}

fn setup_cpus(x: i32) -> Vec<CPU> {
    let mut cpus = Vec::new();
    let mut counter = x;
    while counter != 0 {
        cpus.push(CPU::new(P, R));
        counter -= 1;
    }
    cpus
}

fn clear_cpus(cpus: &mut Vec<CPU>) {
    let mut index = 0;
    while index != cpus.len() {
        let cpu = cpus.get_mut(index).unwrap();
        cpu.clear_load();
        cpu.clear_asked();
        cpu.clear_moved();
        index += 1;
    }
}

fn dispenser(cpus: &mut Vec<CPU>) -> (i32, i32) {
    let mut processes_done = 0;
    let mut rng = thread_rng();
    let mut failed_processes = 0;
    let mut time_passed = 0;

    while processes_done < NO_PROCESSES {
        time_passed += 1;

        if rng.gen_range(1, 101) >= RAND_PROCESS_CHANCE {
            let process = Process::new();
            let mut how_many_asked = 0;
            let mut asked: HashSet<i32> = HashSet::new();
            let mut has_been_added = false;
            let cpu_asking_no = rng.gen_range(0, NO_CPUS);
            asked.insert(cpu_asking_no);
            let mut found_index: i32 = -1;
            while how_many_asked != Z {
                let cpu_number = rng.gen_range(0, NO_CPUS);
                if asked.contains(&cpu_number) {
                    continue;
                }
                let temp_cpu = cpus.get_mut(cpu_number as usize).unwrap();
                if !temp_cpu.is_overloaded() && temp_cpu.can_process(&process) {
                    has_been_added = true;
                    found_index = cpu_number;
                    break;
                } else {
                    asked.insert(cpu_number);
                    how_many_asked += 1;
                }
            }
            if !has_been_added {
                let cpu_asking = cpus.get_mut(cpu_asking_no as usize).unwrap();
                if cpu_asking.can_process(&process) {
                    found_index = cpu_asking_no;
                } else {
                    failed_processes += 1;
                }
            }
            if found_index != -1 {
                cpus.get_mut(found_index as usize).unwrap().add(process);
            }
        }
        let mut iter = 0;
        while iter < NO_CPUS {
            let cpu = cpus.get_mut(iter as usize).unwrap();
            cpu.add_load();
            processes_done += cpu.work();
            iter += 1;
        }
    }
    (failed_processes, time_passed)
}

fn fair_work(cpus: &mut Vec<CPU>) -> (i32, i32) {
    let mut processes_done = 0;
    let mut rng = thread_rng();
    let mut failed_processes = 0;
    let mut time_passed = 0;

    while processes_done < NO_PROCESSES {
        time_passed += 1;
        if rng.gen_range(1, 101) >= RAND_PROCESS_CHANCE {
            let process = Process::new();
            let mut how_many_asked = 0;
            let mut asked: HashSet<i32> = HashSet::new();
            let mut has_been_added = false;
            let cpu_asking_no = rng.gen_range(0, NO_CPUS);
            asked.insert(cpu_asking_no);
            let mut found_index: i32 = -1;
            while how_many_asked != NO_CPUS {
                let cpu_number = rng.gen_range(0, NO_CPUS);
                if asked.contains(&cpu_number) {
                    continue;
                }
                let temp_cpu = cpus.get_mut(cpu_number as usize).unwrap();
                if !temp_cpu.is_overloaded() && temp_cpu.can_process(&process) {
                    found_index = cpu_number;
                    has_been_added = true;
                    break;
                } else {
                    asked.insert(cpu_number);
                    how_many_asked += 1;
                }
            }
            if !has_been_added {
                let cpu_asking = cpus.get_mut(cpu_asking_no as usize).unwrap();
                if cpu_asking.can_process(&process) {
                    found_index = cpu_asking_no;
                } else {
                    failed_processes += 1;
                }
            }
            if found_index != -1 {
                cpus.get_mut(found_index as usize).unwrap().add(process);
            }
        }
        let mut iter = 0;
        while iter < NO_CPUS {
            let cpu = cpus.get_mut(iter as usize).unwrap();
            cpu.add_load();
            processes_done += cpu.work();
            iter += 1;
        }
    }
    (failed_processes, time_passed)
}

fn hardworking(cpus: &mut Vec<CPU>) -> (i32, i32) {
    let mut processes_done = 0;
    let mut rng = thread_rng();
    let mut failed_processes = 0;
    let mut time_passed = 0;

    while processes_done < NO_PROCESSES {
        time_passed += 1;
        if rng.gen_range(1, 101) >= RAND_PROCESS_CHANCE {
            let process = Process::new();
            let mut how_many_asked = 0;
            let mut asked: HashSet<i32> = HashSet::new();
            let mut has_been_added = false;
            let cpu_asking_no = rng.gen_range(0, NO_CPUS);
            asked.insert(cpu_asking_no);
            let mut found_index: i32 = -1;
            while how_many_asked != NO_CPUS {
                let cpu_number = rng.gen_range(0, NO_CPUS);
                if asked.contains(&cpu_number) {
                    continue;
                }
                let temp_cpu = cpus.get_mut(cpu_number as usize).unwrap();
                if !temp_cpu.is_overloaded() && temp_cpu.can_process(&process) {
                    found_index = cpu_number;
                    has_been_added = true;
                    break;
                } else {
                    asked.insert(cpu_number);
                    how_many_asked += 1;
                }
            }
            if !has_been_added {
                let cpu_asking = cpus.get_mut(cpu_asking_no as usize).unwrap();
                if cpu_asking.can_process(&process) {
                    found_index = cpu_asking_no;
                } else {
                    failed_processes += 1;
                }
            }
            if found_index != -1 {
                cpus.get_mut(found_index as usize).unwrap().add(process);
            }
        }
        let mut iter = 0;
        while iter < NO_CPUS && time_passed % 20 == 0 {
            if cpus.get_mut(iter as usize).unwrap().is_underloaded() {
                let mut inner_iter = 0;
                while inner_iter < NO_CPUS {
                    if cpus.get_mut(inner_iter as usize).unwrap().is_overloaded() {
                        let mut temp_vec: Vec<Process> = Vec::new();
                        while cpus.get_mut(inner_iter as usize).unwrap().is_overloaded() {
                            temp_vec.push(cpus.get_mut(inner_iter as usize).unwrap().pop());
                        }
                        while !temp_vec.is_empty() {
                            cpus.get_mut(iter as usize)
                                .unwrap()
                                .add(temp_vec.pop().unwrap());
                        }
                    }
                    inner_iter += 1;
                }
            }
            iter += 1;
        }

        iter = 0;
        while iter < NO_CPUS {
            let cpu = cpus.get_mut(iter as usize).unwrap();
            cpu.add_load();
            processes_done += cpu.work();
            iter += 1;
        }
    }

    (failed_processes, time_passed)
}
