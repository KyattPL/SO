use crate::Process;

#[derive(Debug)]
pub struct Queue {
    pub list: Vec<i32>,
}

impl Queue {
    pub fn new(v: Vec<i32>) -> Queue {
        Queue { list: v }
    }

    pub fn push_process(&mut self, process: i32) {
        self.list.push(process);
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn size(&self) -> usize {
        self.list.len()
    }

    pub fn remove(&mut self, index: usize) {
        self.list.remove(index);
    }

    pub fn insertion_sort(&mut self, processes: &Vec<Process>) {
        let mut counter = 1;
        while counter < self.list.len() {
            let mut inner_counter = counter;
            
            while inner_counter > 0 && processes.get(self.list[inner_counter-1] as usize).unwrap().get_task_time() > processes.get(self.list[inner_counter] as usize).unwrap().get_task_time() {
                let temp_val = self.list[inner_counter - 1];
                self.list[inner_counter - 1] = self.list[inner_counter];
                self.list[inner_counter] = temp_val;
                inner_counter -= 1;
            }
            counter += 1;
        }
    }
}
