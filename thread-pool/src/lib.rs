mod demo;
pub mod linked_list;

use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{self, Receiver, Sender, TryRecvError},
        Arc, Mutex,
    },
    thread::{self},
};

extern crate num_cpus;
struct CommonData {
    name: Option<String>,
    max_thread_count: AtomicUsize,
    queue_count: AtomicUsize,
    active_thread_count: AtomicUsize,
    stack_size: Option<usize>,
    locker: Mutex<()>,
    receiver: Arc<Mutex<Receiver<Task>>>,
}

pub struct ThreadPool {
    common_data: Arc<CommonData>,
    sender: Sender<Task>,
}

type Task = Box<dyn FnOnce() -> () + Send + 'static>;

fn start_thread(common_data: Arc<CommonData>) {
    let mut builder = thread::Builder::new();
    if let Some(ref name) = common_data.name {
        builder = builder.name(name.clone())
    }
    if let Some(ref stack_size) = common_data.stack_size {
        builder = builder.stack_size(stack_size.clone())
    }
    builder
        .spawn(move || loop {
            if common_data.queue_count.load(Ordering::Acquire) == 0 {
                common_data
                    .active_thread_count
                    .fetch_sub(1, Ordering::SeqCst);
                break;
            }
            let result = common_data.receiver.lock().unwrap().try_recv();
            match result {
                Ok(task) => {
                    task();
                    common_data.queue_count.fetch_sub(1, Ordering::SeqCst)
                }
                Err(e) => match e {
                    TryRecvError::Empty => continue,
                    TryRecvError::Disconnected => panic!("error"),
                },
            };
        })
        .unwrap();
}

pub enum ThreadPoolError {}

#[derive(Clone, Default)]
pub struct Builder {
    num_threads: Option<usize>,
    thread_name: Option<String>,
    thread_stack_size: Option<usize>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            num_threads: None,
            thread_name: None,
            thread_stack_size: None,
        }
    }

    pub fn num_threads(mut self, num_threads: usize) -> Builder {
        assert!(num_threads > 0);
        self.num_threads = Some(num_threads);
        self
    }

    pub fn thread_name(mut self, name: String) -> Builder {
        self.thread_name = Some(name);
        self
    }

    pub fn thread_stack_size(mut self, size: usize) -> Builder {
        self.thread_stack_size = Some(size);
        self
    }

    pub fn build(self) -> ThreadPool {
        let (tx, rx) = mpsc::channel::<Task>();
        let num_threads = self.num_threads.unwrap_or_else(num_cpus::get);
        let shared_data = Arc::new(CommonData {
            name: self.thread_name,
            queue_count: AtomicUsize::new(0),
            active_thread_count: AtomicUsize::new(0),
            max_thread_count: AtomicUsize::new(num_threads),
            stack_size: self.thread_stack_size,
            locker: Mutex::new(()),
            receiver: Arc::new(Mutex::new(rx)),
        });

        ThreadPool {
            sender: tx,
            common_data: shared_data,
        }
    }
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> ThreadPool {
        Builder::new().num_threads(num_threads).build()
    }

    pub fn with_name(name: String, num_threads: usize) -> ThreadPool {
        Builder::new()
            .num_threads(num_threads)
            .thread_name(name)
            .build()
    }

    pub fn queue_count(&self) -> usize {
        self.common_data.queue_count.load(Ordering::Relaxed)
    }

    pub fn active_thread_count(&self) -> usize {
        self.common_data.active_thread_count.load(Ordering::SeqCst)
    }

    pub fn max_thead_count(&self) -> usize {
        self.common_data.max_thread_count.load(Ordering::Relaxed)
    }

    pub fn execute<F>(&mut self, f: F) -> Result<(), String>
    where
        F: FnOnce() -> () + Send + 'static,
    {
        if self.common_data.active_thread_count.load(Ordering::Acquire)
            < self.common_data.max_thread_count.load(Ordering::Relaxed)
        {
            let _lock = self.common_data.locker.lock().unwrap();
            if self.common_data.active_thread_count.load(Ordering::Acquire)
                < self.common_data.max_thread_count.load(Ordering::Relaxed)
            {
                start_thread(self.common_data.clone());
                self.common_data
                    .active_thread_count
                    .fetch_add(1, Ordering::SeqCst);
            }
        }
        let result = self.sender.send(Box::new(f));
        self.common_data.queue_count.fetch_add(1, Ordering::SeqCst);
        match result {
            Ok(()) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}
