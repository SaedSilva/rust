use std::cmp::Ordering;
use std::io; // Biblioteca padrão std e io (prelúdio)
use rand::Rng;

//fn declara uma nova função
fn main() {
    //Macro
    println!("Adivinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1..=100);

    println!("Numero secreto: {numero_secreto}");

    loop {
        println!("Digite o seu palpite");

        let mut palpite = String::new();

        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler entrada!");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("Você disse: {palpite}");

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }

    let macas = 5; // imutável
    let mut bananas = 5; // mutável

    let x = 5;
    let y = 10;

    println!("x = {} and y + 2 = {}", x, y + 2);
}
