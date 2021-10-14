extern crate system_shutdown;

use rand::Rng;
use std::io;
use system_shutdown::shutdown;

fn main() {
    println!("digite um nuemro de 1 a 6:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler sla ;/");

    let value: i32 = input.trim().parse().unwrap();
    let value2 = rand::thread_rng().gen_range(1..6);
    if value == value2 {
        match shutdown() {
            Ok(_) => println!("desligando kk"),
            Err(_) => println!("erro ao desliga sla ;/"),
        }
    } else {
        println!("deu sorte irmao ;)");
    }
}
