pub mod elf;

use crate::isr;
use crate::mem;
use crate::util::StaticCell;

const MAX_TASKS: usize = 2;

static TASKS: StaticCell<[Option<Task>; MAX_TASKS]> = StaticCell::new([None; MAX_TASKS]);
// we assume that at least one task (id=0) is created
static CUR: StaticCell<usize> = StaticCell::new(0);

global_asm!(include_str!("switch.S"));

extern "C" {
    fn task_switch(prev: &mut KernelState, next: &KernelState);
    static task_start: u8;
}

/// Returns a reference to the `Task` with given id or `None`
pub fn get(id: usize) -> Option<&'static Task> {
    TASKS[id].as_ref()
}

/// Returns the current `Task`
pub fn current() -> &'static mut Task {
    TASKS.get_mut()[*CUR].as_mut().unwrap()
}

/// Creates a new `Task` with given entry point (typically from the ELF binary).
pub fn create(entry: usize) -> usize {
    let id = TASKS.iter().position(|t| t.is_none()).unwrap();
    TASKS.get_mut()[id] = Some(Task::new(id, entry));
    id
}

fn next_task() -> Option<usize> {
    // simple round robin scheduler
    let mut pos = (*CUR + 1) % MAX_TASKS;
    while pos != *CUR {
        // if the task exists, pick it
        if let Some(_) = get(pos) {
            return Some(pos);
        }
        pos = (pos + 1) % MAX_TASKS;
    }
    None
}

/// Switches to the next task
pub fn schedule() {
    // if there is another task to run, switch to it, otherwise continue with the current one
    if let Some(next) = next_task() {
        get(next).unwrap().make_current();
    }
}

/// Represents the state for in-kernel context switches. Used in task_switch to save and restore
/// the callee save CPU registers.
#[derive(Copy, Clone, Default)]
#[repr(C, packed)]
struct KernelState {
    ebx: usize,
    esp: usize,
    ebp: usize,
    esi: usize,
    edi: usize,
    eflags: usize,
}

/// A tasks is an entity that can execute code and thus contains a kernel stack, a user stack, and a
/// `KernelState` to pause and resume its execution.
#[derive(Copy, Clone)]
pub struct Task {
    id: usize,
    kstack: usize,
    ustack: usize,
    entry: usize,
    kstate: KernelState,
}

impl Task {
    fn new(id: usize, entry: usize) -> Self {
        let mut kstate = KernelState::default();

        // init register state
        let kstack = mem::alloc_page();
        let ustack = mem::alloc_page();
        kstate.esp = kstack + mem::PAGE_SIZE;
        kstate.ebp = kstate.esp;
        kstate.esi = entry;
        kstate.edi = ustack + mem::PAGE_SIZE;

        // init stack
        let top_idx = mem::PAGE_SIZE / 4 - 1;
        kstate.esp = kstack + top_idx * 4;
        let kstack_ptr = kstack as *mut usize;
        unsafe {
            // that's the address we're going to return to in the first call of task_switch
            *kstack_ptr.offset(top_idx as isize) = &task_start as *const u8 as usize;
        }

        Self {
            id,
            kstack,
            ustack,
            entry,
            kstate,
        }
    }

    /// Returns the id
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns the entry point
    pub fn entry(&self) -> usize {
        self.entry
    }

    /// Returns the top end (stack grows downwards) of the kernel stack
    pub fn kstack_end(&self) -> usize {
        self.kstack + mem::PAGE_SIZE
    }

    /// Returns the top end (stack grows downwards) of the user stack
    pub fn ustack_end(&self) -> usize {
        self.ustack + mem::PAGE_SIZE
    }

    /// Performs a context switch to this task
    pub fn make_current(&self) {
        let cur = current();
        // set TSS::sp0 as the kernel stack pointer when we enter the kernel again
        isr::set_entry_sp(self.kstack_end());
        // this task is running now
        CUR.set(self.id);
        // save state of old task and restore state of this task
        unsafe {
            task_switch(&mut cur.kstate, &self.kstate);
        }
        // we don't return here directly, but only later when we resume the old task
    }
}
