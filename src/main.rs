use menemen::request::{Request, RequestTypes};
use std::{fs::File, io};

fn main() {
    let mut request = Request::new(
        "https://behemehal.org/img/bBrand/main.png",
        RequestTypes::GET,
    )
    .unwrap();

    match request.send() {
        Ok(mut cevap) => {
            let mut file = File::create("./resim.png").unwrap();
            io::copy(&mut cevap.stream, &mut file).unwrap();
        }
        Err(e) => {
            println!("Bağlantı Hatası: {:?}", e);
        }
    }
}