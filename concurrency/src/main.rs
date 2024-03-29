use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let (tx,rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals= vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),    
        ];
    
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals= vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),    
        ];
    
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }



    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        println!("Here is a vector {:?}", v);
        thread::sleep(Duration::from_millis(1));
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();
}


