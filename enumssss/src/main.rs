fn main() {
    let quatro = VersaoIp::V4(String::from(""));
    let seis = VersaoIp::V6;

    rotear(quatro);

    let local = EnderecoIp::V4(127, 0, 0, 1);

    let loopback = EnderecoIp::V6(String::from("::1"));


    let algum_numero = Some(5);
    let algum_texto = Some("um texto");
    let numero_ausente: Option<i32> = None;

}

fn rotear(versao_ip: VersaoIp) {
    println!("{:#?}", versao_ip)
}


#[derive(Debug)]
enum VersaoIp {
    V4(String),
    V6(String)
}

enum EnderecoIp {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Mensagem {
    Sair,
    Mover { x: i32, y: i32 },
    Escrever(String),
    MudarCor(i32, i32, i32),
}

impl Mensagem {
    fn invocar(&self) {
        // o corpo do método é definido aqui
    }
}
