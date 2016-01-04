use std::thread;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
    left: usize,
    right: usize
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {  // "associated function" new, create new instances of structs
        Philosopher {                                               // "Philosopher" is the last expression of this function, this is returned
            name: name.to_string(),                                 // create copy of string that &str points to
            left: left,
            right: right
        }
    }
    
    fn eat(&self, table:&Table) {                                   // explicit "self", this defines a method as apposed to an "associated function"
        let _left = table.forks[self.left].lock().unwrap();         // lock() will block a mutex currently being accessed to someone else
        thread::sleep_ms(150);                                      // unwrap() will cover cases where lock() may fail, so mutex becomes available again
        let _right = table.forks[self.right].lock().unwrap();       // use leading underscore to indicate we just want to reference value inside the lock, not use it
        
        println!("{} is eating.", self.name);
        
        thread::sleep_ms(1000);
        
        println!("{} is done eating", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>       // Mutex is a way to control concurrency
}

fn main() {
    let table = Arc::new(Table { forks: vec![   // Arc: "atomic reference count"
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(())
    ]});
    
    let philosophers = vec![
        Philosopher::new("Judith Butler",    0, 1),
        Philosopher::new("Gilles Deleuze",   1, 2),
        Philosopher::new("Karl Marx",        2, 3),
        Philosopher::new("Emma Goldman",     3, 4),
        Philosopher::new("Michael Foucault", 0, 4)      // left handed philosopher solves this problem
    ];
    
    let handles: Vec<_> = philosophers.into_iter().map(|p| {    // "_" is a type placeholder
        let table = table.clone();
        
        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();
    
    for h in handles {
        h.join().unwrap();      // join() blocks execution until the thread has completed execution
    }
}
