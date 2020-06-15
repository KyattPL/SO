mod cpu;
mod process;

use cpu::CPU;
use process::Process;
use rand::*;

const P: i32 = 70;
const R: i32 = 30;
const Z: i32 = 10;
const NO_CPUS: i32 = 50;
const NO_PROCESSES: i32 = 100_000;
const RAND_PROCESS_CHANCE: i32 = 90;

fn main() {
    let mut cpus = setup_cpus(NO_CPUS);
    let (mut processes, mut processes2, mut processes3) = generate_processes();

    let (fails, time_passed) = dispenser(&mut cpus, &mut processes);
    println!("No failed processes: {}", fails);
    let global_avg = avg_load(&mut cpus, time_passed);
    standard_deviation(&mut cpus, global_avg, time_passed);

    println!();
    clear_cpus(&mut cpus);

    let (fails, time_passed) = fair_work(&mut cpus, &mut processes2);
    println!("No failed processes: {}", fails);
    let global_avg = avg_load(&mut cpus, time_passed);
    standard_deviation(&mut cpus, global_avg, time_passed);

    println!();
    clear_cpus(&mut cpus);

    let (fails, time_passed) = hardworking(&mut cpus, &mut processes3);
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
        println!(
            "CPU no {}, avg load: {}, asked: {}, given : {}",
            iter + 1,
            avg,
            temp_cpu.get_asked(),
            temp_cpu.get_given()
        );
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
        cpu.clear_given();
        index += 1;
    }
}

fn generate_processes() -> (
    Vec<(i32, Process)>,
    Vec<(i32, Process)>,
    Vec<(i32, Process)>,
) {
    let mut procs = Vec::new();
    let mut procs2 = Vec::new();
    let mut procs3 = Vec::new();
    let mut processes_done = 0;
    let mut rng = rand::thread_rng();
    while processes_done != NO_PROCESSES {
        let cpu_no = rng.gen_range(0, NO_CPUS);
        procs.push((cpu_no, Process::new()));
        procs2.push((cpu_no, Process::new()));
        procs3.push((cpu_no, Process::new()));
        processes_done += 1;
    }
    (procs, procs2, procs3)
}

fn dispenser(cpus: &mut Vec<CPU>, processes: &mut Vec<(i32, Process)>) -> (i32, i32) {
    let mut processes_done = 0;
    let mut rng = thread_rng();
    let mut failed_processes = 0;
    let mut time_passed = 0;

    while processes_done < NO_PROCESSES {
        time_passed += 1;

        if rng.gen_range(1, 101) >= (100 - RAND_PROCESS_CHANCE) && !processes.is_empty() {
            let (cpu_asking_no, process) = processes.pop().unwrap();
            let mut how_many_asked = -1;
            let mut cpu_pool: Vec<i32> = vec![0; NO_CPUS as usize]
                .into_iter()
                .map(|_x| {
                    how_many_asked += 1;
                    how_many_asked
                })
                .collect();
            cpu_pool.remove(cpu_asking_no as usize);
            how_many_asked = 0;
            let mut has_been_added = false;
            let mut found_index: i32 = -1;
            while how_many_asked != Z {
                how_many_asked += 1;
                let choice = rng.gen_range(0, cpu_pool.len());
                let cpu_number = cpu_pool.get(choice).unwrap();
                let temp_cpu = cpus.get_mut(*cpu_number as usize).unwrap();
                if !temp_cpu.is_overloaded() && temp_cpu.can_process(&process) {
                    has_been_added = true;
                    found_index = *cpu_number;
                    break;
                }
                cpu_pool.remove(choice as usize);
            }
            if !has_been_added {
                let cpu_asking = cpus.get_mut(cpu_asking_no as usize).unwrap();
                cpu_asking.ask(how_many_asked);
                if cpu_asking.can_process(&process) {
                    found_index = cpu_asking_no;
                } else {
                    failed_processes += 1;
                    processes_done += 1;
                }
            } else {
                let cpu_asking = cpus.get_mut(cpu_asking_no as usize).unwrap();
                cpu_asking.ask(how_many_asked);
                cpu_asking.give();
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

fn fair_work(cpus: &mut Vec<CPU>, processes: &mut Vec<(i32, Process)>) -> (i32, i32) {
    let mut processes_done = 0;
    let mut rng = thread_rng();
    let mut failed_processes = 0;
    let mut time_passed = 0;

    while processes_done < NO_PROCESSES {
        time_passed += 1;
        if rng.gen_range(1, 101) >= (100 - RAND_PROCESS_CHANCE) && !processes.is_empty() {
            let (cpu_asking_no, process) = processes.pop().unwrap();
            let mut how_many_asked = -1;
            let mut cpu_pool: Vec<i32> = vec![0; NO_CPUS as usize]
                .into_iter()
                .map(|_x| {
                    how_many_asked += 1;
                    how_many_asked
                })
                .collect();
            how_many_asked = 0;
            let mut has_been_added = false;
            let mut found_index: i32 = -1;
            cpu_pool.remove(cpu_asking_no as usize);
            while how_many_asked != (NO_CPUS - 1) {
                how_many_asked += 1;
                let choice = rng.gen_range(0, cpu_pool.len());
                let cpu_number = cpu_pool.get(choice).unwrap();
                let temp_cpu = cpus.get_mut(*cpu_number as usize).unwrap();
                if !temp_cpu.is_overloaded() && temp_cpu.can_process(&process) {
                    found_index = *cpu_number;
                    has_been_added = true;
                    break;
                }
                temp_cpu.give();
                cpu_pool.remove(choice as usize);
            }
            if !has_been_added {
                let cpu_asking = cpus.get_mut(cpu_asking_no as usize).unwrap();
                if cpu_asking.can_process(&process) {
                    found_index = cpu_asking_no;
                } else {
                    failed_processes += 1;
                    processes_done += 1;
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

fn hardworking(cpus: &mut Vec<CPU>, processes: &mut Vec<(i32, Process)>) -> (i32, i32) {
    let mut processes_done = 0;
    let mut rng = thread_rng();
    let mut failed_processes = 0;
    let mut time_passed = 0;

    while processes_done < NO_PROCESSES {
        time_passed += 1;
        if rng.gen_range(1, 101) >= (100 - RAND_PROCESS_CHANCE) && !processes.is_empty() {
            let (cpu_asking_no, process) = processes.pop().unwrap();
            let mut how_many_asked = -1;
            let mut cpu_pool: Vec<i32> = vec![0; NO_CPUS as usize]
                .into_iter()
                .map(|_x| {
                    how_many_asked += 1;
                    how_many_asked
                })
                .collect();
            how_many_asked = 0;
            let mut has_been_added = false;
            let mut found_index: i32 = -1;
            cpu_pool.remove(cpu_asking_no as usize);
            while how_many_asked != (NO_CPUS - 1) {
                how_many_asked += 1;
                let choice = rng.gen_range(0, cpu_pool.len());
                let cpu_number = cpu_pool.get(choice).unwrap();
                let temp_cpu = cpus.get_mut(*cpu_number as usize).unwrap();
                if !temp_cpu.is_overloaded() && temp_cpu.can_process(&process) {
                    found_index = *cpu_number;
                    has_been_added = true;
                    break;
                }
                temp_cpu.give();
                cpu_pool.remove(choice as usize);
            }
            if !has_been_added {
                let cpu_asking = cpus.get_mut(cpu_asking_no as usize).unwrap();
                if cpu_asking.can_process(&process) {
                    found_index = cpu_asking_no;
                } else {
                    failed_processes += 1;
                    processes_done += 1;
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
                            cpus.get_mut(inner_iter as usize).unwrap().give();
                        }
                        while !temp_vec.is_empty() {
                            cpus.get_mut(iter as usize)
                                .unwrap()
                                .add(temp_vec.pop().unwrap());
                        }
                    }
                    cpus.get_mut(iter as usize).unwrap().ask(1);
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
