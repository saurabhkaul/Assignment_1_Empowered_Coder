use std::fmt::format;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::thread;
use std::time::Duration;
use log::warn;


fn main() {
    let (tx, rx) = mpsc::channel();
    let mut tasks:Vec<Task> = vec![];

    
    for i in 1..10{
        tasks.push(Task{
            id: i,
            payload: format!("This is task number {i}"),
        })
    }

    for task in tasks {
        let worker = Worker::create_worker(task.id,tx.clone()); //adding task id for now
        worker.process_task(task)
    }
    drop(tx); //ensuring theres no senders

    //Print all processed tasks by workers

    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(received) => println!("Received Output from Worker -> {}", received),
            Err(e) => {
                println!("timeout limit crossed, e: {e}");
                break;
            }
        }
    }

    // while let Ok(received) rx.recv_timeout(Duration::from_secs(1)){
    //     println!("Received Output from Worker -> {}", received)
    // }

    // rx.recv_timeout(Duration::from_secs(1)
    // let mut iter = rx.recv_timeout();
    //
    // loop {
    //     match iter.next() {
    //         Some(value) => {
    //             println!("Received Output from Worker -> {}", value);
    //         }
    //         None => {
    //             println!("No more values");
    //             break;
    //         }
    //     }

}

struct Task{
    id:u32,
    payload: String,
}

fn create_task(id: u32, payload: &str) -> Task{
    Task{
        id,
        payload: payload.to_string(),
    }
}

struct Worker{
    id:u32,
    tx: Sender<String>,
}

impl Worker {
    fn create_worker(id: u32,tx:Sender<String>) -> Worker{
        Worker{
            id,
            tx
        }
    }
    fn process_task(&self, task: Task){
        let (task_id,payload) = (task.id,task.payload);
        let id  = self.id;
        let tx = self.tx.clone();

        //all odd id tasks take longer to process
        match id % 2 == 0 {
            true => {
                thread::spawn(move ||{
                    let _ = tx.send(format!("Task {task_id} processed. Payload: {payload}. Worker {id}"));

                });
            }
            false => {
                thread::spawn(move ||{
                    thread::sleep(Duration::from_secs(2));
                    let _ = tx.send(format!("Task {task_id} processed. Payload: {payload}. Worker {id}"));
                });

            }
        }
    }
}

