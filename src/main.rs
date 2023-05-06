use std::io;
use std::process::Command;
use std::sync::mpsc::{channel, RecvError};
use std::thread;
use std::time;
use threadpool::ThreadPool;

use clap::Parser;

// fn task() {
//     let cmd = "echo hello"; // Replace with your shell command
//                             // Execute the shell command
//     let output = Command::new("sh")
//         .arg("-c")
//         .arg(cmd)
//         .output()
//         .expect("failed to execute process");

//     thread::sleep(time::Duration::from_millis(1000));

//     // Print the output to the console
//     println!("Thread \n{}", String::from_utf8_lossy(&output.stdout));
// }

// fn main() {
//     // let cmd = "echo hello"; // Replace with your shell command

//     let pool = ThreadPool::new(num_cpus::get());
//     // let (tx, rx) = channel();

//     // Spawn 10 threads
//     // let mut handles = Vec::new();
//     for _ in 0..100 {
//         // let handle = thread::spawn(move || {
//         //     // Execute the shell command
//         //     let output = Command::new("sh")
//         //         .arg("-c")
//         //         .arg(cmd)
//         //         .output()
//         //         .expect("failed to execute process");

//         //     thread::sleep(time::Duration::from_millis(1000));

//         //     // Print the output to the console
//         //     println!("Thread {}:\n{}", i, String::from_utf8_lossy(&output.stdout));
//         // });

//         pool.execute(move || task());

//         // let handle = thread::spawn(move || task());
//         // handles.push(handle);
//     }

//     // for handle in handles {
//     //     handle.join().unwrap();
//     // }
// }

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// command to run
    #[arg(short, long)]
    spark: String,

    /// command to run
    #[arg(long)]
    spread: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    threads: usize,
}

use rayon::prelude::*;

fn exc_spread_cmd(x: String, cmd: String) {
    let cmd = format!("{} {}", cmd.clone(), x.clone());

    // Execute the shell command
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .arg(x)
        .output()
        .expect("failed to execute process");

    thread::sleep(time::Duration::from_millis(1000));

    println!("{:?}", output);

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Thread \n{}", stdout);
    } else {
        println!("Thread \n{}", String::from_utf8_lossy(&output.stderr));
    }
}

fn exc_spark_cmd(cmd: String) -> Result<Vec<String>, String> {
    // Execute the shell command
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");

    println!("{:?}", output);

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Thread \n{}", stdout);

        let lines: Vec<String> = stdout.trim().split('\n').map(|x| x.to_string()).collect();

        println!("{:?}", lines);
        return Ok(lines.clone());
    } else {
        println!("Thread \n{}", String::from_utf8_lossy(&output.stderr));
        Err("too young".to_string())
    }
}

fn main() {
    let args = Args::parse();

    // let results: Vec<_> = (0..100).into_par_iter().map(|_| task()).collect();

    let sparks = exc_spark_cmd(args.spark);
    println!("OUTPUT {:?}", sparks);

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build()
        .unwrap();

    let (tx, rx) = std::sync::mpsc::channel();
    for s in sparks.unwrap().into_iter() {
        let tx = tx.clone();
        let spread_cmd = args.spread.clone();
        pool.spawn(move || {
            tx.send(exc_spread_cmd(s.clone(), spread_cmd)).unwrap();
        });
    }
    drop(tx); // need to close all senders, otherwise...
    let hashes: Vec<_> = rx.into_iter().collect(); // ... this would block

    // sparks
    //     .unwrap()
    //     .into_par_iter()
    //     .for_each(|x| exc_spread_cmd(x, args.spread.clone()));
}
