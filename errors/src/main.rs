use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tentou criar um arquivo e houve um problema: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "Houve um problema ao abrir o arquivo: {:?}",
                error
            )
        },
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}