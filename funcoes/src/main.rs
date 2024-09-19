fn main() {
    println!("Hello, world!");
    outra_funcao(4, 6);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("O valor de y é: {}", y);

    let x = cinco();

    println!("O valor de x é: {}", x);
}

fn outra_funcao(x: i32, y: i32) {
    println!("O valor de x é: {}", x);
    println!("O valor de y é: {}", y);
}

fn cinco() -> i32 {
    5
}
