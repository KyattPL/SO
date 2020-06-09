use crate::process::Process;

pub struct CPU {
    p: i32,
    r: i32,
    current_load: i32,
    processes: Vec<Process>,
    sum_of_load: i32,
    count_asked: i32,
    count_moved: i32,
}

impl CPU {
    pub fn new(p: i32, r: i32) -> CPU {
        CPU {
            p,
            r,
            current_load: 0,
            processes: Vec::new(),
            sum_of_load: 0,
            count_asked: 0,
            count_moved: 0,
        }
    }

    pub fn is_overloaded(&self) -> bool {
        self.current_load > self.p
    }

    pub fn is_underloaded(&self) -> bool {
        self.current_load < self.r
    }

    pub fn pop(&mut self) -> Process {
        self.processes.pop().unwrap()
    }

    pub fn can_process(&self, proc: &Process) -> bool {
        100 >= self.current_load + proc.required_power
    }

    pub fn add(&mut self, proc: Process) {
        self.current_load += proc.required_power;
        self.processes.push(proc);
    }

    pub fn add_load(&mut self) {
        self.sum_of_load += self.current_load;
    }

    pub fn get_sum_of_load(&self) -> i32 {
        self.sum_of_load
    }

    pub fn clear_load(&mut self) {
        self.sum_of_load = 0;
    }

    pub fn clear_asked(&mut self) {
        self.count_asked = 0;
    }

    pub fn clear_moved(&mut self) {
        self.count_moved = 0;
    }

    pub fn work(&mut self) -> i32 {
        let mut size = self.processes.len();
        let mut done = 0;
        let mut counter = 0;
        while counter < size {
            let temp_proc = self.processes.get_mut(counter).unwrap();
            temp_proc.time_left -= 1;
            if temp_proc.time_left == 0 {
                self.current_load -= temp_proc.required_power;
                self.processes.remove(counter);
                done += 1;
                size = self.processes.len();
                continue;
            }
            counter += 1;
        }
        done
    }
}
