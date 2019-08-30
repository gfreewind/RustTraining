use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    #[derive(Debug)]
    struct TestStruct1 {
        x1: i32,
        x2: i32,
    };

    /* Box<T> */
    let t1 = Box::new(TestStruct1{x1:1, x2:2});
    println!("t1 is {:?}", t1);
    let t2 = t1;
    //println!("t1 is {:?}", t1);
    println!("t2 is {:?}", t2);

    /* Rc<T> immutable */
    let t3 = Rc::new(TestStruct1{x1:1, x2:2});
    println!("t3 is {:?}, strong_cnt is {}", t3, Rc::strong_count(&t3));
    let t4 = t3.clone();
    println!("t3 is {:?}, strong_cnt is {}", t3, Rc::strong_count(&t3));
    println!("t4 is {:?}, strong_cnt is {}", t4, Rc::strong_count(&t4));
    //t3.x1 = 4;

    // Refcell<T>
    let t5 = Rc::new(RefCell::new(TestStruct1{x1:1, x2:2}));
    println!("t5 is {:?}, strong_cnt is {}", t5, Rc::strong_count(&t5));
    let t6 = t5.clone();
    println!("t6 is {:?}, strong_cnt is {}", t6, Rc::strong_count(&t6));
    let t7 = t5.borrow();
    println!("t7 is {:?}, strong_cnt is {}", t7, Rc::strong_count(&t5));
    let t8 = t5.borrow();
    println!("t8 is {:?}", t8);

    /*
    let t9 = t5.borrow_mut();
    let t10 = t6.borrow_mut();
    println!("t9 is {:?}", t9);
    println!("t10 is {:?}", t10);
    */
}
