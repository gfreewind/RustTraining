fn main() {

    // uninitialized 
    println!("*********************** example1 *****************************");
    let x1: i32 = 1;
    let px: &i32 = &x1;

    println!("x1 is {}, px is {}", x1, px);

    // use after free
    println!("*********************** example2 *****************************");
    {
        let x2:i32 = 2;
        //px = &x2; 
        println!("x2 is {}", x2);
    }
    println!("px is {}", px);

    // overflow
    println!("*********************** example3 *****************************");
    let mut x3:[i32; 3] = [0;3];
    for i in 0..3 {
        x3[i] = 3;
    }
    println!("x3 is {:?}", x3);

    // double free
    println!("*********************** example4 *****************************");
    let pt1: Box<i32>;
    let pt2 = Box::new(12);

    pt1 = pt2;
    //println!("pt1 is {}, pt2 is {}", pt1, pt2);
    println!("pt1 is {}", pt1);
}
