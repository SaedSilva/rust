fn main() {
    let lista_numero = vec![34, 50, 25, 100, 65];

    let resultado = maior_i32(&lista_numero);
    println!("O maior número {}", resultado);

    let lista_char = vec!['y', 'm', 'a', 'q'];

    let resultado = maior_char(&lista_char);
    println!("O maior char é {}", resultado);

    let inteiro = Ponto { x: 5, y: 10 };
    let float = Ponto { x: 1.0, y: 4.0 };
}

fn maior_i32(lista: &[i32]) -> i32 {
    let mut maior = lista[0];

    for &item in lista.iter() {
        if item > maior {
            maior = item;
        }
    }

    maior
}

fn maior_char(lista: &[char]) -> char {
    let mut maior = lista[0];

    for &item in lista.iter() {
        if item > maior {
            maior = item;
        }
    }

    maior
}

struct Ponto<T> {
    x: T,
    y: T,
}