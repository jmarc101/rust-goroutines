use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    thread,
};

type Task = Box<dyn FnOnce() + Send + 'static>;
type Queue = Arc<(Mutex<VecDeque<Task>>, Condvar)>;

struct Worker {
    queue: Queue,
}

pub struct Scheduler {
    queues: Arc<Mutex<Vec<Queue>>>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl Scheduler {
    pub fn new(num_threads: usize) -> Arc<Self> {
        let mut workers = Vec::with_capacity(num_threads);
        let queues = Arc::new(Mutex::new(Vec::new()));

        for _ in 0..num_threads {
            let q = Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));
            {
                let mut guard = queues.lock().unwrap();
                guard.push(q.clone());
            }

            let queues_ref = queues.clone();
            let handle = thread::spawn(move || {
                loop {
                    let task_opt = {
                        let (lock, cv) = &*q;
                        let mut guard = lock.lock().unwrap();

                        loop {
                            if let Some(task) = guard.pop_back() {
                                break Some(task);
                            }

                            let mut stolen: Option<Task> = None;
                            for (_, other_q) in queues_ref.lock().unwrap().iter().enumerate() {
                                if Arc::ptr_eq(other_q, &q) {
                                    continue;
                                }

                                let (olock, _) = &**other_q;
                                if let Ok(mut oguard) = olock.lock() {
                                    if let Some(t) = oguard.pop_front() {
                                        stolen = Some(t);
                                        break;
                                    }
                                }
                            }

                            if stolen.is_some() {
                                break stolen;
                            }

                            guard = cv.wait(guard).unwrap();
                        }
                    };

                    if let Some(task) = task_opt {
                        task();
                    }
                }
            });

            workers.push(handle);
        }

        Arc::new(Self { workers, queues })
    }
}

macro_rules! go {
    ($body:expr) => {
        thread::spawn(move || $body)
    };
}

fn add(x: u8, y: u8) -> u8 {
    let result = x + y;
    println!("{}", result);
    return result;
}

fn main() {
    let name = "Jean-Marc".to_string();
    let age = 30;

    let go_name = name.clone();
    // Spawn a "goroutine-like" thread
    let handle = go!({
        println!(
            "Hello from goroutine-like thread: {} is {} years old",
            go_name, age
        );
    });

    go!(add(1, 2));

    println!("{}", name);

    handle.join().unwrap();
}
