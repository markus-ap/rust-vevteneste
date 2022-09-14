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

    let (status, filnamn) = if spørrelinje == "GET / HTTP/1.1"{
        ("HTTP/1.1 200 OK", "indeks.html")
    }
    else if spørrelinje.contains("/hei"){
        ("HTTP/1.1 200 OK", "hei.index")
    }
    else{
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    
    let innhald = fs::read_to_string(filnamn).unwrap();
    let lengde = innhald.len();
    let svar = format!("{status}\r\nContent-Length: {lengde}\r\n\r\n{innhald}");
    
    straum.write_all(svar.as_bytes()).unwrap();
}
