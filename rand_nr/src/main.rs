use rand::Rng;

fn main() {
    let rand_nr = rand::thread_rng().gen_range(1, 100);

    println!("Hello, world! Random number is {}", rand_nr);
}
