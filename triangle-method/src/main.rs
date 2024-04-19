use std::io;

fn main() {
    println!("Triagle method!");

    let mut a = String::new();
    let mut b = String::new(); 
    let mut n = String::new();
    let mut h = String::new(); 

    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");
    let a: f32 = a.trim().parse().expect("Please type a number!");
    
    io::stdin()
        .read_line(&mut b)
        .expect("Failed to read line");
    let b: f32 = b.trim().parse().expect("Please type a number!");

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let mut n: i8 = n.trim().parse().expect("Please type a number!");

    io::stdin()
        .read_line(&mut h)
        .expect("Failed to read line");
    let h: f32 = h.trim().parse().expect("Please type a number!");

    let h2 = h/2.0;

    while n !=0 {



        n -= 1;
    }

    println!("a = {a}");
    println!("b = {b}");
    println!("n = {n}");
    println!("h = {h}");
    println!("h/2 = {h2}");

    
}
