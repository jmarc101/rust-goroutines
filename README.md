# Goroutines in Rust (from scratch)

This project re-implements the idea of **Go goroutines** in **Rust**, step by step.  
The goal is to show Go developers what the Go runtime does for them automatically, by manually building similar abstractions in Rust.

---

## 🎯 Goal

- Build a tiny "goroutine runtime" in Rust.
- Provide a `go!(...)` macro similar to Go’s `go func(...)`.
- Explore how goroutines differ from OS threads.
- Learn what Go’s runtime does under the hood:
  - green threads (lightweight tasks)
  - scheduler
  - work-stealing thread pool
  - communication via channels
  - cooperative multitasking (yield points)

---

## 🛠 Planned Steps

1. **Hello world with Rust threads**  
   - show `std::thread::spawn` (1:1 mapping to OS threads).
   - contrast with Go’s lightweight goroutines.

2. **Build a `go!(...)` macro**  
   - abstracts thread spawning at first.
   - so we can write `go!(|| { println!("hi"); });`.

3. **Introduce a task system**  
   - tasks are stored in a queue.  
   - a scheduler runs tasks on a thread pool.  
   - tasks can yield voluntarily.

4. **Add channels**  
   - re-implement Go’s `chan` with `std::sync::mpsc`.  
   - eventually extend with async channels.

5. **Show cooperative scheduling**  
   - tasks yield back to the scheduler.  
   - multiple tasks run on fewer OS threads.  
   - simulate how goroutines multiplex.

6. **Compare with real Go**  
   - show how tiny our runtime is vs Go’s runtime.  
   - explain extra features Go adds (garbage collection, stack growth, work-stealing, etc).

---

## 📚 Talking Points for Go Devs

- In Go:  
  - `go func()` is magic.  
  - scheduler, stack management, channels are built-in.  

- In Rust:  
  - you must build (or pick) a runtime.  
  - no garbage collector = memory safety at compile time.  
  - async/await exists, but we’ll build a goroutine-like runtime manually.

---

## ▶️ Usage Example

Eventually we’ll be able to write:

```rust
fn main() {
    go!(|| {
        println!("hello from goroutine 1");
    });

    go!(|| {
        println!("hello from goroutine 2");
    });

    std::thread::sleep(std::time::Duration::from_secs(1));
}
```

and it will behave like Go.
