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
}
