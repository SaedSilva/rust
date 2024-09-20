fn main() {
    let mut x = 5;
    println!("O valor de x é: {}", x);
    x = 6;
    println!("O valor de x é: {}", x);


    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("O valor de x é: {}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("O valor de tup é: {:?}", tup);

    let (x, y, z) = tup;

    println!("O valor do y é: {}", y);

    let a = [1, 2, 3, 4, 5];

    println!(type(a));
}