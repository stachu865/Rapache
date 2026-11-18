use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fs::File;
fn handle_url(url: &str) -> String {
// 1. Odetnij query string (wszystko po '?')
let path = url.split('?').next().unwrap_or("/");
// 2. Odetnij fragment (wszystko po '#')
let path = path.split('#').next().unwrap_or("/");
// 3. Usuń wiodący '/'
let path = path.trim_start_matches('/');
// 4. Pusta ścieżka → index.html
let path = if path.is_empty() { "index.html" } else { path };
// 5. URL-decode (obsługa %20, %C4%85 itd.)
let path = url_decode(path);
// 6. Zabezpieczenie przed "../"
if path.contains("..") {
    return "index.html".to_string(); // albo lepiej: zwróć błąd
}
// 7. Doklej katalog bazowy
format!("public/{}", path)
}
fn url_decode(s: &str) -> String {
let bytes = s.as_bytes();
let mut out = Vec::with_capacity(bytes.len());
let mut i = 0;
while i < bytes.len() {
    if bytes[i] == b'%' && i + 2 < bytes.len() {
        let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or("");
        if let Ok(b) = u8::from_str_radix(hex, 16) {
            out.push(b);
            i += 3;
            continue;
        }
    }
    out.push(bytes[i]);
    i += 1;
}
String::from_utf8_lossy(&out).into_owned()
}
fn handle_client(mut stream: TcpStream) {
let mut buffer = [0; 1024];
// Odczytaj żądanie HTTP (ignorujemy zawartość)
let _ = stream.read(&mut buffer);
let first_line = String::from_utf8_lossy(&buffer);
let mut parts = first_line.split_whitespace();
let _method = parts.next();
let raw_url = parts.next().unwrap_or("/");
let path = handle_url(raw_url);
let mut body = File::open(path)
    .unwrap();
let mut  str_buff = String::new();    
let _ = body.read_to_string(&mut str_buff);
str_buff = format!("HTTP/1.1 200 OK\r\n\r\n{}", str_buff);
let _ = stream.write_all(str_buff.as_bytes());
let _ = stream.flush();

// KLUCZOWE: jawnie zamknij połączenie
//let _ = stream.shutdown(std::net::Shutdown::Both);
}

fn main() -> std::io::Result<()> {
let listener = TcpListener::bind("0.0.0.0:80")
    .expect("erroe");
println!("Serwer nasłuchuje na http://localhost");
    
for stream in listener.incoming() {
    match stream {
        Ok(stream) => {
            thread::spawn(|| {
                handle_client(stream);
            });
        }
        Err(e) => {
            eprintln!("Błąd połączenia: {}", e);
        }
    }
}
Ok(())
}