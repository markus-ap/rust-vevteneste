use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};

fn main() {
    let lytter = TcpListener::bind("127.0.0.1:7878").unwrap();

    for straum in lytter.incoming(){
        let straum = straum.unwrap();
        handter_kobling(straum);
    }
}

fn handter_kobling(mut straum: TcpStream){
    let buf_leser = BufReader::new(&mut straum);

    let spørrelinje = buf_leser.lines().next().unwrap().unwrap();
    let svar;

    if spørrelinje == "GET / HTTP/1.1"{
        let status = "HTTP/1.1 200 OK";
        let innhald = fs::read_to_string("indeks.html").unwrap();
        let lengde = innhald.len();
    
        svar = format!("{status}\r\nContent-Length: {lengde}\r\n\r\n{innhald}");
    }
    else if spørrelinje.contains("/hei") {
        let status = "HTTP/1.1 200 OK";
        let innhald = fs::read_to_string("hei.html").unwrap();     
        let lengde = innhald.len();   
        svar = format!("{status}\r\nContent-Length: {lengde}\r\n\r\n{innhald}");
    }    
    else{
        let status = "HTTP/1.1 404 NotFound";
        let innhald = fs::read_to_string("404.html").unwrap();
        let lengde = innhald.len();
        svar = format!("{status}\r\nContent-Length: {lengde}\r\n\r\n{innhald}");
    }
    
    straum.write_all(svar.as_bytes()).unwrap();
}
