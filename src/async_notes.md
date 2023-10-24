## Async Notes

- async functions must be in a async runtime
- most runtimes start with a "block_on()" call
  - block_on is how you relinquish control to the async runtime
- you can launch multiple threads and spawn multiple runtimes inside
- futures are basically promises in js

##### example of block_on

```rs
use futures::executor::block_on;

async fn im_async(){
    println!("I'm Async");
}

fn main(){
    println!("I'm not Async"); // normal sync runtime
    block_on(im_async()); // control relinquished to the async runtime
    println!("I'm not Async"); // normal sync runtime
}
```

---

## JOIN

Tokio crate provides a join! macro that takes multiple async functions and runs them at the same time

```rs

println!("I'm not Async"); // normal sync runtime
// tokio runs all at once and when all are done return them as a tuple
let result: (i32, i32) = tokio::join!(get_i32_async(), get_i32_async());
println!("I'm not Async"); // normal sync runtime
```

---

## SPAWN

We can use tokio::spawn to launch a detached task that we dont have to wait for it to come back

```rs
async fn tick(){
    for i:i32 in 0..100 {
        println!("tick {i}")
    }
}
async fn tock(){
    for i:i32 in 0..100 {
        println!("tock {i}")
    }
}

#[tokio::main]
async fn main(){
    let _ = tokio::join!(
    tokio::spawn(tick()),
    tokio::spawn(tock()),
    );
}
```

---

## SELECT

Process whatever returns first

```rs
use rand::Rng;

async fn randomized_return(){
    let rng = rand::thread_rng();
    let seconds = rng.gen_range(0..9);
    tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;
}

#[tokio::main]{
    async fn main(){
        for _ in 0..10 {
            tokio::select! {
                _ = randomized_return() => println!("A"),
                _ = randomized_return() => println!("B"),
                _ = randomized_return() => println!("C"),
            }
        }
    }
}
```

---

## YIELD CONTROL

Sometimes on of our async task is expected to take a long time we can use yield
to move it to the end of the queue starting back up when all other task are done

```rs
async fn long_task(){
    for i in 0..10 {
        println!("time is ticking")
        tokio::task::yield_now().await
    }
}
async fn normal_task(){
    for i in 0..10 {
        println!("time is ticking")
        tokio::task::yield_now().await
    }
}

#[tokio::main]
async fn main(){
let _ = tokio::join!(
    tokio::spawn(long_task()),
    tokio::spawn(normal_task()),
    )
}
```

---

## BLOCKING WITH SLEEP

Sometimes a task might be blocking causing the whole runtime to wait for that task
to be completed. One such example is std::thread::sleep . This will put the whole
runtime to sleep basically. Tokio provides a solution

```rs
// ------- ✗ BAD for ASYNC ✗ ----------------------
std::thread::sleep(dur:Duration::from_millis(time));

// ------- ✔ GOOD for ASYNC ✔ ----------------------
tokio::time::sleep(Duration::from_millis(time)).await;
```

---

### spawn_blocking

use tokio::task::spawn_blocking for spawning asynchronous task, this will sit in
a thread created by tokio. This function is executed outside the main event loop,
so it doesn't affect the overall responsiveness of your program. When the blocking
operation is done, it returns a result that you can use. Imagine there is a slow
complicated job to be done, you can get a specialist to do that job while the
rest of the team continues to work on the main task.

```rs
async fn slow_job() {
    spawn_blocking(move || {/*something slow*/}).await.unwrap()
}
```
