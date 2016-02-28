use std::thread;
use lib::consts; // internal module

pub mod lib;

fn main()
{
    println!("test result {}", test(6));
    
    let mut children = vec![];
    let mut done     = false;

    for i in 0..consts::NTHREADS
    {
        // create thread
        let child = thread::spawn(move ||
        {
            let mut biere = 0;
            println!("this is thread number {}", i);
            while !done
            {
                if biere == 100000 { done = true }
                biere += 1
            }
            println!("thread {} : cave remplie", i);
        });
        // Spin up another thread
        children.push(child);
    }

    for child in children
    {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}

fn test(x: i32) -> i32 {
    return x + 1;
}
