use std::thread;
use std::sync::Arc;
use std::sync::mpsc;
use std::time::Duration;
use std::sync::Mutex;

fn main() {
    let v = vec![1, 2, 3];
    println!("vec is {:?}", v);

    /* ownership check */
    println!("*********************** example1 *****************************");
    //let thr = thread::spawn({
    let thr = thread::spawn(move || {
        println!("thread print vec is {:?}", v);
    });

    //println!("main pring vec {:?} again", v);
    thr.join().unwrap();

    /* channel & ownership */
    println!("*********************** example2 *****************************");
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
    println!("*********************** example3 *****************************");
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

    // mutex usage, never foget lock
    println!("*********************** example4 *****************************");
    #[derive(Debug)]
    struct Test {
        x: i32,
        y: i32,
    }
    let m = Mutex::new(Test{x:1, y:2});
    let mut t = m.lock().unwrap();
    t.x = 2;
    t.y = 3;
    println!("t is {:?}", t);

    //mutex with multiple threads
    println!("*********************** example5 *****************************");
    let counter = Arc::new(Mutex::new(0));
    let mut thrs = vec![];

    for _ in 0..10 {
        let thr_ounter = Arc::clone(&counter);
        let thr = thread::spawn(move || {
            let mut nr = thr_ounter.lock().unwrap();
            *nr = *nr + 1;
        });
        thrs.push(thr);
    }

    for thr in thrs {
        thr.join().unwrap();
    }
    println!("Count is {}", counter.lock().unwrap());

}
