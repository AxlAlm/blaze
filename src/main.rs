use clap::Parser;
use rayon::prelude::*;
use serde_json::Value;
use std::process::Command;
use std::thread;
use std::time;

mod person;
use person::Person;

use quick_protobuf::{BytesReader, MessageRead, Writer};

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
    cmd: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 2)]
    threads: usize,
}

fn exc_spread_cmd(x: String, cmd: String) {
    let cmd = format!("{} '{}'", cmd.clone(), x.clone());

    println!("{:?}", cmd);

    // Execute the shell command
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        println!("Thread \n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("Thread \n{}", String::from_utf8_lossy(&output.stderr));
    }

    thread::sleep(time::Duration::from_millis(5000));
}

// fn exc_spark_cmd(cmd: String) -> Result<Vec<String>, String> {
//     // Execute the shell command
//     let output = Command::new("sh")
//         .arg("-c")
//         .arg(cmd)
//         .output()
//         .expect("failed to execute process");

//     if output.status.success() {
//         let stdout = String::from_utf8_lossy(&output.stdout);
//         let lines: Vec<String> = stdout.trim().split('\n').map(|x| x.to_string()).collect();
//         return Ok(lines.clone());
//     } else {
//         Err("oh no".to_string())
//     }
// }

fn read_json_file(filepath: &str) -> Result<String, String> {
    let text = std::fs::read_to_string(&filepath).unwrap();
    // let data = serde_json::from_str::<Value>(&text).unwrap();
    return Ok(text);
}

// fn main() {
//     let args = Args::parse();

//     let data = vec![read_json_file("inputs/abc.json").unwrap()];
//     println!("{:?}", data);

//     // let sparks = exc_spark_cmd(args.spark);
//     // println!("{:?}", rayon::max_num_threads());

//     let meta_pool = rayon::ThreadPoolBuilder::new()
//         .num_threads(2)
//         .build()
//         .unwrap();

//     let worker_pool = rayon::ThreadPoolBuilder::new()
//         .num_threads(args.threads)
//         .build()
//         .unwrap();

//     // let (tx, rx) = std::sync::mpsc::channel();

//     meta_pool.scope(move |s| {
//         s.spawn(move |_| loop {
//             println!("{:?}", "hello");
//             thread::sleep(time::Duration::from_millis(1000));
//         });

//         worker_pool.scope(move |s| {
//             for x in data.into_iter() {
//                 // let tx = tx.clone();
//                 let spread_cmd = args.cmd.clone();
//                 s.spawn(move |_| {
//                     println!("{:?}", "start");
//                     exc_spread_cmd(x.clone(), spread_cmd);
//                     println!("{:?}", "done");
//                 });
//             }
//         });
//     });

//     // worker_pool.scope(move |worker_scope| {
//     //     for s in sparks.unwrap().into_iter() {
//     //         let tx = tx.clone();
//     //         let spread_cmd = args.spread.clone();
//     //         worker_scope.spawn(move |_| {
//     //             tx.send(exc_spread_cmd(s.clone(), spread_cmd)).unwrap();
//     //         });
//     //     }
//     // });
//     // let _: Vec<_> = rx.into_iter().collect(); // ... this would block

//     // perpetual_pool.install(move || loop {
//     //     println!("{:?}", "hello");
//     //     thread::sleep(time::Duration::from_millis(1000));
//     // });

//     // perpetual_pool.spawn(move || loop {
//     //     println!("{:?}", "hello");
//     //     thread::sleep(time::Duration::from_millis(1000));
//     // });

//     // let n = pool.install(|| fib(20));

//     // pool.install(|| {
//     //     for batch in db_items_rx {
//     //         batch.into_par_iter().try_for_each(|mut item| {
//     //             let result = scanner.reprocess(&mut item)?;
//     //             processed_tx.send(processed_item)
//     //             Ok(())
//     //         })?;
//     //     }

//     //     Ok(())
//     // })?;

//     // sparks
//     //     .unwrap()
//     //     .par_iter()
//     //     .for_each(|s| exc_spread_cmd(s.clone(), args.spread.clone()));

//     // {
//     //     let spread_cmd = args.spread.clone();
//     //     pool.spawn(move || {
//     //         exc_spread_cmd(s.clone(), spread_cmd);
//     //     });
//     // }

//     // for s in sparks.unwrap().into_iter() {
//     //     let tx = tx.clone();
//     //     let spread_cmd = args.spread.clone();
//     //     pool.spawn(move || {
//     //         tx.send(exc_spread_cmd(s.clone(), spread_cmd)).unwrap();
//     //     });
//     // }

//     // drop(tx);

//     // need to close all senders, otherwise...

//     // sparks
//     //     .unwrap()
//     //     .into_par_iter()
//     //     .for_each(|x| exc_spread_cmd(x, args.spread.clone()));
// }

fn main() {
    // bytes is a buffer on the data we want to deserialize
    // typically bytes is read from a `Read`:
    // r.read_to_end(&mut bytes).expect("cannot read bytes");
    let mut bytes: Vec<u8>;
    bytes = vec![];

    // we can build a bytes reader directly out of the bytes
    let mut reader = BytesReader::from_bytes(&bytes);

    // now using the generated module decoding is as easy as:
    let p = Person::from_reader(&mut reader, &bytes).expect("Cannot read FooBar");

    // if instead the buffer contains a length delimited stream of message we could use:
    // while !r.is_eof() {
    //     let foobar: FooBar = r.read_message(&bytes).expect(...);
    //     ...
    // }
    println!("{:?}", p);
}
