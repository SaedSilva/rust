fn main() {
    let cinco = Some(5);
    let seis = mais_um(cinco);
    let nenhum = mais_um(None);

    let algum_valor_u8 = 0u8;
    match algum_valor_u8 {
        1 => println!("um"),
        3 => println!("três"),
        5 => println!("cinco"),
        7 => println!("sete"),
        _ => (),
    }

    let algum_valor_u8 = Some(0u8);
    match algum_valor_u8 {
        Some(3) => println!("três"),
        _ => (),
    }

    if let Some(3) = algum_valor_u8 {
        println!("três");
    }
}


fn mais_um(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn valor_em_cents(moeda: Moeda) -> u32 {
    match moeda {
        Moeda::Penny => 1,
        Moeda::Nickel => 5,
        Moeda::Dime => 10,
        Moeda::Quarter => 25,
    }
}

enum Moeda {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
