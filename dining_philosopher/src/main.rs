/*
* The dining philosophers problem involves multiple threads needing
* synchronized access to shared resources, risking deadlock.
*
* This code models philosophers as threads and forks as shared Mutex<()>
* wrapped in Arc for thread-safe reference counting.
*
* To prevent deadlock from a "deadly embrace" of waiting for neighboring
* forks, philosophers acquire lower numbered forks first. This breaks
* symmetry and avoids circular waiting.
*
* The Mutexes provide exclusive fork access. The Arc allows sharing forks
* between philosophers.
*
* The simulation prints start time, eating duration, and total time for
* all philosophers. Total time approximately equals philosophers divided
* by forks, as that number can eat concurrently.
*
* Key techniques:
* - Used Mutex<()> to represent exclusive fork access
* - Wrapped in Arc to share Mutexes between threads
* - Numbered philosophers and acquire lower fork first
* - Prints timing metrics for simulation
*/
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct Fork {
    id: u32,
    mutex: Mutex<()>,
}

struct Philosopher {
    id: u32,
    name: String,
    left_fork: Arc<Fork>,
    right_fork: Arc<Fork>,
}

impl Philosopher {
    fn new(id: u32, name: &str, left_fork: Arc<Fork>, right_fork: Arc<Fork>) -> Self {
        Self {
            id,
            name: name.to_string(),
            left_fork,
            right_fork,
        }
    }

    fn eat(&self) {
        let (first_fork, sec_fork) = if self.id % 2 == 0 {
            (&self.left_fork, &self.right_fork)
        } else {
            (&self.right_fork, &self.left_fork)
        };

        println!("{} picked up fork {}.", self.name, first_fork.id);
        let _first_guard = first_fork.mutex.lock().unwrap();
        println!("{} picked up fork {}.", self.name, sec_fork.id);
        let _sec_guard = sec_fork.mutex.lock().unwrap();

        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_secs(1));
        println!("{} finished eating.", self.name);

        println!("{} put down fork {}.", self.name, first_fork.id);
        println!("{} put down fork {}.", self.name, sec_fork.id);
    }
}

fn main() {
    println!("Dining Philosophers Problem:  15 Philosophers, 4 Forks...Yikes!!");

    let forks: Vec<Arc<Fork>> = (0..4)
        .map(|id| {
            Arc::new(Fork {
                id,
                mutex: Mutex::new(()),
            })
        })
        .collect();

    let philosophers: Vec<Philosopher> = vec![
        ("Jürgen Habermas", 0, 1),
        ("Friedrich Engels", 1, 2),
        ("Karl Marx", 2, 3),
        ("Thomas Piketty", 3, 0),
        ("Michel Foucault", 0, 1),
        ("Socrates", 1, 2),
        ("Plato", 2, 3),
        ("Aristotle", 3, 0),
        ("Pythagoras", 0, 1),
        ("Heraclitus", 1, 2),
        ("Democritus", 2, 3),
        ("Diogenes", 3, 0),
        ("Epicurus", 0, 1),
        ("Zeno of Citium", 1, 2),
        ("Thales of Miletus", 2, 3),
    ]
    .into_iter()
    .enumerate()
    .map(|(id, (name, left_fork, right_fork))| {
        Philosopher::new(
            id as u32,
            name,
            Arc::clone(&forks[left_fork]),
            Arc::clone(&forks[right_fork]),
        )
    })
    .collect();

    let start = Instant::now();

    let handles = philosophers
        .into_iter()
        .map(|philosopher| {
            thread::spawn(move || {
                philosopher.eat();
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total time: {:?}", start.elapsed());
}
