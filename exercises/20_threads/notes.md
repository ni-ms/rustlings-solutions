This Rust program is based on **concurrent programming** using **threads**, which allows multiple tasks to execute
simultaneously.

### **Key Concepts in This Code**

1. **Spawning Threads**
    - The program uses `thread::spawn()` to create **multiple** threads (10 in this case).
    - Each thread executes a function that tracks its execution time.
    - The `move` keyword ensures the closure captures necessary variables.

2. **Sleeping in Threads**
    - `thread::sleep(Duration::from_millis(250))` forces each thread to pause for 250 milliseconds.
    - This simulates work being done.

3. **Capturing Execution Time**
    - `Instant::now()` is used to **record the start time** of a thread.
    - `start.elapsed().as_millis()` calculates how long each thread took to complete.

4. **Joining Threads** (TODO)
    - Currently, the program **does not collect** the return values of threads.
    - The missing piece is calling `.join().unwrap()` on each `JoinHandle` to retrieve the return value.
    - Without this, the program does not correctly ensure **all threads complete before moving forward**.

5. **Handling Results**
    - The program intends to store the results in a `Vec<u128>` but hasn’t implemented the collection yet.
    - The check `if results.len() != 10` ensures that all threads are accounted for.

6. **Thread Safety Considerations**
    - Since each thread runs independently, Rust prevents data races by ensuring captured values are moved or safely
      accessed.

### **What's Missing?**

- The loop needs to **collect** the results by joining each thread handle.
- Without joining, threads might still be running when the main thread checks `results.len()`, leading to a panic.
- Implementing `.join().unwrap()` properly ensures synchronization.

This program is a great example of **parallel execution and synchronization** in Rust.

Note:

If you are using collect, we need to manually infer the type.
We need to manually infer type for anything that might return multiple collections (~collections in java?)
Don't forget what the map function does!!
For any function call, keep a note of whether it takes ownership of the object.

```rust
thread::spawn(move | | {
p.eat();
})
```

Here’s where the concurrency happens. The thread::spawn function takes a closure as an argument and executes that
closure in a new thread. This closure needs an extra annotation, move, to indicate that the closure is going to take
ownership of the values it’s capturing. Primarily, the p variable of the map function.

```rust
for h in handles {
h.join().unwrap();
}
```

At the end of main(), we loop through the handles and call join() on them, which blocks execution until the thread has
completed execution. This ensures that the threads complete their work before the program exits.

```rust
fn eat(&self, table: &Table) {
    let _left = table.forks[self.left].lock().unwrap();
    let _right = table.forks[self.right].lock().unwrap();

    println!("{} is eating.", self.name);

    thread::sleep_ms(1000);

    println!("{} is done eating.", self.name);
}
```

We have two new lines. We’ve also added an argument, table. We access the Table’s list of forks, and then use self.left
and self.right to access the fork at that particular index. That gives us access to the Mutex at that index, and we call
lock() on it. If the mutex is currently being accessed by someone else, we’ll block until it becomes available.

The call to lock() might fail, and if it does, we want to crash. In this case, the error that could happen is that the
mutex is ‘poisoned’, which is what happens when the thread panics while the lock is held. Since this shouldn’t happen,
we just use unwrap().

```rust
use std::thread;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        thread::sleep_ms(1000);

        println!("{} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}

```

You need to wrap the `Table` struct in an `Arc` (Atomic Reference Counted pointer) because the table is shared among multiple philosopher threads.

### Why use `Arc`?
Each philosopher thread needs access to the same table instance, specifically the `Vec<Mutex<()>>` that represents the forks. However, ownership rules in Rust dictate that data should only have one owner unless it's specifically wrapped in a smart pointer designed for shared access. `Arc` ensures safe shared ownership across multiple threads.

### What happens if you don't use `Arc`?
If you simply define `table` as a `Table` instance without wrapping it in `Arc`, when you try to move it into multiple threads, Rust's ownership system would prevent compilation. The table would be **moved** into the first thread, making it inaccessible to others. `Arc` allows multiple threads to own the same instance of `Table` while tracking references, ensuring safe memory management.

Both `Arc` (Atomic Reference Counted) and `Rc` (Reference Counted) are smart pointers that provide shared ownership of heap-allocated data in Rust, but they differ in how they handle concurrency.

### **Key Differences:**
1. **Thread Safety**
   - `Rc<T>` **is not** thread-safe. It does not use atomic operations, making it more efficient but unsafe for sharing across threads.
   - `Arc<T>` **is** thread-safe. It uses atomic reference counting, allowing multiple threads to safely share ownership of data.

2. **Performance**
   - `Rc<T>` is faster because it avoids the overhead of atomic operations. This makes it ideal for single-threaded applications.
   - `Arc<T>` incurs extra cost due to atomic operations but is necessary for multi-threaded programs.

3. **Use Cases**
   - **Use `Rc<T>`** when you need shared ownership **within** a single thread.
   - **Use `Arc<T>`** when you need shared ownership **across multiple threads**.

### **Example of Rc (Single-Threaded):**
```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(5);
    let b = Rc::clone(&a);
    println!("a = {}, b = {}", a, b);
}
```

### **Example of Arc (Multi-Threaded):**
```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let a = Arc::new(5);
    let b = Arc::clone(&a);

    thread::spawn(move || {
        println!("b = {}", b);
    }).join().unwrap();
}
```

If you're working on multi-threaded applications (like your dining philosophers example!), `Arc<T>` is the way to go. If you're in a single-threaded context, `Rc<T>` offers better performance.

In Rust, `.unwrap()` is a method used to extract the value inside a `Result<T, E>` or `Option<T>` type.

### **How does `.unwrap()` work?**
- If the value is **`Ok(T)`** or **`Some(T)`**, `.unwrap()` returns the value `T`.
- If the value is **`Err(E)`** or **`None`**, `.unwrap()` causes a **panic** and terminates the program.

### **Your example (`handle.join().unwrap()`)**
In your case, `handle.join()` returns a `Result<(), Box<dyn Any + Send>>`, which means the thread **could potentially panic** during execution. By calling `.unwrap()`, you are saying:
> "I expect this thread to run successfully. If it panics, terminate the program."

### **Safer Alternatives**
Instead of `.unwrap()`, you can handle errors gracefully:
```rust
for handle in handles {
    match handle.join() {
        Ok(_) => println!("Thread finished successfully."),
        Err(e) => println!("Thread panicked: {:?}", e),
    }
}
```
This prevents an abrupt crash and lets you log or recover from errors.

# Threads 2
https://doc.rust-lang.org/book/ch16-03-shared-state.html#atomic-reference-counting-with-arct

# Threads 3
https://doc.rust-lang.org/book/ch16-02-message-passing.html

Clone the tx and use it.