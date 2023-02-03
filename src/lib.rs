//use glam::{Vec2, Vec3Swizzles};

use std::{
    cell::UnsafeCell,
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub mod camera;
pub mod geometry;
pub mod mesh;
pub mod model;
pub mod texture;
pub mod transform;
pub mod utils;
pub use {
    camera::Camera,
    geometry::*,
    mesh::Mesh,
    model::Model,
    texture::Texture,
    transform::{Transform, TransformInitialParams},
    utils::*,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let builder = thread::Builder::new();
        let thread = builder
            .spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                //println!("Worker {id} got a job; executing.");
                job();
            })
            .expect("Could not spawn new thread!");

        Worker { id, thread }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
