use crate::Request;

pub struct Queue {
    pub list: Vec<i32>,
}

impl Queue {
    pub fn new(v: Vec<i32>) -> Queue {
        Queue { list: v }
    }

    pub fn push_request(&mut self, request: i32) {
        self.list.push(request);
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

    pub fn insertion_sort(&mut self, head_pos: i32, requests: &mut Vec<Request>) {
        let mut counter = 1;
        while counter < self.list.len() {
            let mut inner_counter = counter;
            let mut pos1 = requests.get(self.list[inner_counter-1] as usize).unwrap().distance_to_head(head_pos);
            let mut pos2 = requests.get(self.list[inner_counter] as usize).unwrap().distance_to_head(head_pos);

            while inner_counter > 1 && pos1 > pos2 {
                let temp_val = self.list[inner_counter - 1];
                self.list[inner_counter - 1] = self.list[inner_counter];
                self.list[inner_counter] = temp_val;
                inner_counter -= 1;
                pos1 = requests.get(self.list[inner_counter-1] as usize).unwrap().distance_to_head(head_pos);
                pos2 = requests.get(self.list[inner_counter] as usize).unwrap().distance_to_head(head_pos);
            }
            counter += 1;
        }
    }
}
