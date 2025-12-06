use std::{
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle},
};

pub struct WorkProcessor<T> {
    sender: mpsc::Sender<Message<T>>,
    worker: Vec<JoinHandle<()>>,
}

enum Message<T> {
    Task(T),
    Stop,
}

impl<T: Send + 'static> WorkProcessor<T> {
    pub fn new<F>(task_fn: F) -> Self
    where
        F: Fn(T) + Send + Sync + 'static,
    {
        let (tx, rx) = mpsc::channel::<Message<T>>();
        let rx = Arc::new(Mutex::new(rx));

        let mut worker = Vec::new();

        let task_fn = Arc::new(task_fn);

        let cpu_cores = num_cpus::get();

        for thread_id in 1..=cpu_cores {
            let rx_cloned = rx.clone();
            let task_fn_cloned = task_fn.clone();

            let handle = thread::spawn(move || {
                loop {
                    let msg = rx_cloned.lock().unwrap().recv();

                    let fix = match msg {
                        Ok(msg) => msg,
                        Err(e) => {
                            eprint!("{}", e);
                            return;
                        }
                    };

                    match fix {
                        Message::Task(data) => {
                            task_fn_cloned(data);
                            println!("Thread {} is processing a task", thread_id);
                        }
                        Message::Stop => break,
                    };
                }
            });

            worker.push(handle);
        }
        Self {
            sender: tx,
            worker: worker,
        }
    }

    pub fn push(&self, task: T) {
        let _ = self.sender.send(Message::Task(task));
    }

    pub fn stop(self) {
        for _ in &self.worker {
            let _ = self.sender.send(Message::Stop);
        }

        for handler in self.worker {
            let _ = handler.join();
        }
    }
}