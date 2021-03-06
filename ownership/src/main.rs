fn main() {
    // ownership out of scope
    println!("*********************** example1 *****************************");
    {
        let s = "hello";

        println!("s is {}", s);
    }
    //println!("s is {}", s);

    // ownership moved
    println!("*********************** example2 *****************************");
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("s1 is {}, s2 is {}", s1, s2);
    println!("s2 is {}", s2);

    let i1 = 3;
    let i2 = i1;

    println!("i1 is {}, i2 is {}", i1, i2);

    // clone
    println!("*********************** example3 *****************************");
    let x = 5;
    let y = x;
    println!("x is {}, y is {}", x, y);
    {
        let mut s1 = String::from("hello");
        let s2 = s1.clone();
        s1.push_str(", world");
        println!("s1 is {}, s2 is {}", s1, s2);
    }

    // move example2
    println!("*********************** example4 *****************************");
    let mut s3 = String::from("hello");
    //show_str(s3);
    s3 = show_str2(s3);
    show_str3(&s3);
    println!("s3 is {}", s3);

    // double mut referenc
    println!("*********************** example5 *****************************");
    let r1 = &s3;
    let r2 = &s3;
    //let r3 = &mut s3;
    println!("r1 is {}, r2 is {}", r1, r2);
    //println!("r3 is {}, s3 is {}", r3, s3);

    // dangle pointer
    println!("*********************** example6 *****************************");
    let s4 = no_dangle_ref();
    //let s4 = dangle_ref();
    println!("s4 is {}", s4);
}

#[allow(dead_code)]
fn show_str(s:String) {
    println!("s is {}", s);
}

#[allow(dead_code)]
fn show_str2(s:String) -> String {
    println!("s is {}", s);
    s
}

#[allow(dead_code)]
fn show_str3(s: &String) {
    println!("s is {}", s);
}

/*
fn dangle_ref() -> &String {
    let s = String::from("hello");
    &s
}
*/

fn no_dangle_ref() -> String {
    let s = String::from("hello");
    s
}
