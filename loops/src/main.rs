fn main() {
    let mut numero = 3;

    while numero != 0 {
        println!("{}!", numero);

        numero = numero - 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut indice = 0;

    while indice < 5 {
        println!("O valor é: {}", a[indice]);

        indice = indice + 1;
    }

    for elemento in a.iter() {
        println!("O valor é: {}", elemento);
    }

    for numero in (1..4).rev() {
        println!("{}!", numero);
    }

    println!("LIFTOFF!!!");
}
