use core::ptr;
use crate::task;

pub struct Semaphore {
    counter: u32,
    waiting_task: Option<usize>,
}

impl Semaphore {
    pub const fn new(counter: u32) -> Self {
        Self {
            counter,
            waiting_task: None,
        }
    }

    pub fn up(&mut self) {
        self.counter += 1;

        // if there is someone waiting, unblock the task
        if let Some(waiting) = self.waiting_task.take() {
            task::get_mut(waiting).and_then(|t| Some(t.unblock()));
        }
    }

    pub fn down(&mut self) {
        // wait until the counter is non-zero; use read_volatile to ensure that the compiler reads
        // from memory in each loop iteration
        while unsafe { ptr::read_volatile(&self.counter) } == 0 {
            self.waiting_task = Some(task::current().id());
            task::current().block();
            self.waiting_task = None;
        }

        self.counter -= 1;
    }
}
