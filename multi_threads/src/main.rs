use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];
    println!("vec is {:?}", v);

    /* ownership check */
    //let thr = thread::spawn({
    let thr = thread::spawn(move || {
        println!("thread print vec is {:?}", v);
    });

    //println!("main pring vec {:?} again", v);
    thr.join().unwrap();

    /* channel & ownership */
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let val = String::from("Hi, i'm a thread");
        tx1.send(val).unwrap();
        //println!("thread: now val is {}", val);
    });

    let val = rx.recv().unwrap();
    println!("main receive: {}", val);

    /* multiple sender, one receive */
    let tx2 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("tx2: 1"),
            String::from("tx2: 2"),
            String::from("tx2: 3"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let tx3 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("tx3: 1"),
            String::from("tx3: 2"),
            String::from("tx3: 3"),
        ];
        for val in vals {
            tx3.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    drop(tx);

    for receive in rx {
        println!("main get: {}", receive);
    }

}
