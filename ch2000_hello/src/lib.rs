use std::thread;

pub struct Threadpool{
    threads: Vec<thread::JoinHandle<()>>
}

impl Threadpool {
    /// Creates a new Thread pool of  the size specified  in the
    ///  size parameter.
    /// 
    /// The size is the number of threads in the pool
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Threadpool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);

        for i in 0..size {
            //  TODO create thread and store them in vector
        }

        Threadpool {
            threads
        }
    }

    pub fn execute<F>(&self, f: F)
    
    where
        F: FnOnce() + Send + 'static,
    {

    }
}
