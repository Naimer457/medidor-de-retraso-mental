use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("CREADO POR NAIMER, V1.1");
    println!("Por favor introduzca su nombre");
    let mut nombre = String::new();
    io::stdin()
        .read_line(&mut nombre)
        .expect("Failed to read line");
    let x: u128 = rand::thread_rng().gen_range(0..99999999999999999999999999999999999999);
    println!("el nivel de retraso mental de {} es de {}",nombre, x);
}