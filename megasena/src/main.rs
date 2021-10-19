use rand::Rng;

fn main() {
    println!("MEGASENA");
    let mut palpite: [i8;6] = [0;6];

    for i in 0..6 {
        palpite[i] = rand::thread_rng().gen_range(1..61);
    }
    println!("{:?}",palpite);
}
