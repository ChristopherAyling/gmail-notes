use native_tls::{TlsConnector, TlsStream};

use mailparse::*;

fn main() {
    println!("Hello, world!");
    let tls = TlsConnector::builder().build().unwrap();
    
    let client = imap::connect(
        ("imap.gmail.com", 993),
        "imap.gmail.com",
        &tls,
    ).unwrap();

    let username = std::env::var("USERNAME").unwrap();
    let password = std::env::var("PASSWORD").unwrap();

    let mut imap_session = client.login(username, password).unwrap();

    imap_session.select("[Gmail]/All Mail").unwrap();
}

