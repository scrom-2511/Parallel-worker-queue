# Parallel-worker-queue

`Parallel-worker-queue` is a lightweight, CPU-parallel task executor implemented using Rust’s standard library.  
It provides a minimal thread-pool–like system that distributes tasks across all available CPU cores without relying on external crates (other than `num_cpus` for core detection).

## Features

- Spawns one worker thread per CPU core  
- Distributes tasks via an `mpsc` channel  
- Executes a user-provided function for each task  
- Graceful shutdown using `Stop` messages  
- Fully generic over any `Send + 'static` task type

---

## How It Works

When you create a `WorkProcessor`, you supply a function:

```rust
F: Fn(T) + Send + Sync + 'static
```

This function is executed every time a task is submitted.

Internally:

- Each worker thread waits on a shared `mpsc::Receiver<Message<T>>`
- Tasks are wrapped in `Message::Task(T)`
- Shutdown is triggered by sending `Message::Stop`
- Workers exit cleanly after processing all messages
- The main thread can block on worker completion via `stop()`

---

## Usage Example

```rust
fn main() {
    let processor = WorkProcessor::new(|num: u32| {
        println!("Processing number: {}", num);
    });

    for i in 0..10 {
        processor.push(i);
    }

    processor.stop();
}
```

---

## API

### `new(task_fn)`
Creates a new `WorkProcessor` and spawns as many threads as there are CPU cores.

### `push(task)`
Sends a task into the queue to be processed by the next available worker.

### `stop()`
Sends a `Stop` message to all workers and waits for all threads to finish before returning.

