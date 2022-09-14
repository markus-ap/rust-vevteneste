use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    collections::HashMap
};

fn main() {
    let lytter = TcpListener::bind("127.0.0.1:7878").unwrap();    

    for straum in lytter.incoming(){
        let straum = straum.unwrap();

        thread::spawn(|| {
            handter_kobling(straum);
        });
    }
}

fn handter_kobling(mut straum: TcpStream){
    let buf_leser = BufReader::new(&mut straum);
    let spørrelinje = buf_leser.lines().next().unwrap().unwrap();
    if spørrelinje.contains("GET /"){ handter_henting(&spørrelinje, straum); }
}

fn handter_henting(spørrelinje: &String, mut straum: TcpStream){
    use regex::Regex;

    let sidar = HashMap::from([
        ("", "indeks.html"),
        ("hei", "hei.html"),
    ]);

    let mønster = Regex::new(r"GET /(\w*) HTTP/1.1").unwrap();
    let fanget = mønster.captures(&spørrelinje).unwrap();
    let fanget = &fanget[1];
    
    let (status, filnamn) = if !sidar.contains_key(fanget){
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    }
    else{
        let side = sidar[fanget];
        ("HTTP/1.1 200 OK", side)
    };

    let innhald = fs::read_to_string(filnamn).unwrap();
    let svar = lag_svar(status, innhald);
    
    straum.write_all(svar.as_bytes()).unwrap();
}

fn lag_svar(status: &str, innhald: String) -> String{
    let lengde = innhald.len();
    format!("{status}\r\nContent-Length: {lengde}\r\n\r\n{innhald}")
}
