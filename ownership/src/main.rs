fn main() {
    // ownership out of scope
    {
        let s = "hello";

        println!("s is {}", s);
    }
    //println!("s is {}", s);

    // ownership moved
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("s1 is {}, s2 is {}", s1, s2);
    println!("s2 is {}", s2);

    // clone
    let x = 5;
    let y = x;
    println!("x is {}, y is {}", x, y);
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 is {}, s2 is {}", s1, s2);
    }

    // move example2
    let mut s3 = String::from("hello");
    //show_str(s3);
    s3 = show_str2(s3);
    show_str3(&s3);
    println!("s3 is {}", s3);

    // double mut referenc
    let r1 = &s3;
    let r2 = &s3;
    //let r2 = &mut s3;
    println!("r1 is {}, r2 is {}", r1, r2);

    // dangle pointer
    let s4 = no_dangle_ref();
    //let s4 = dangle_ref();
    println!("s4 is {}", s4);
}

#[allow(dead_code)]
fn show_str(s:String) {
    println!("s is {}", s);
}

fn show_str2(s:String) -> String {
    println!("s is {}", s);
    s
}

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
