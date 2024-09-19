fn main() {
    let numero = 3;

    if numero < 5 {
        println!("condição era verdadeira");
    } else {
        println!("condição era falsa");
    }

    let numero = 6;

    if numero % 4 == 0 {
        println!("número é divisível por 4");
    } else if numero % 3 == 0 {
        println!("número é divisível por 3");
    } else if numero % 2 == 0 {
        println!("número é divisível por 2");
    } else {
        println!("número não é divisível por 4, 3 ou 2");
    }

    let condicao = true;
    let numero = if condicao {
        5
    } else {
        6
    };

    println!("O valor do número é: {}", numero);
}