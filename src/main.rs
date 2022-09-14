use std::{
    fs,
    io,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    collections::HashMap,
    ffi::OsStr, hash::Hash
};

use regex::Regex;

fn main() {
    let lytter = TcpListener::bind("127.0.0.1:7878").unwrap();    

    for straum in lytter.incoming(){
        let straum = straum.unwrap();

        thread::spawn(|| {
            let html = hent_html().unwrap();
            handter_kobling(straum, &html);
        });
    }
}

fn handter_kobling(mut straum: TcpStream, sidar: &HashMap<String, String>){
    let buf_leser = BufReader::new(&mut straum);
    let spørrelinje = buf_leser.lines().next().unwrap().unwrap();
    
    if spørrelinje.contains("GET /"){ 
        handter_henting(&spørrelinje, straum, &sidar); 
    }
    else if spørrelinje.contains("POST /"){
        handter_tillegging(&spørrelinje, straum);
    }
}

fn handter_tillegging(spørrelinje: &String, mut straum: TcpStream){
    println!("Mottok POST-spørring.");
    // let mønster = Regex::new(r"POST /(\w*) HTTP/1.1").unwrap();
    // let fanget = mønster.captures(&spørrelinje).unwrap();
    // let fanget = &fanget[1];
    straum.write_all("Hei".as_bytes()).unwrap();
}

fn handter_henting(spørrelinje: &String, mut straum: TcpStream, sidar: &HashMap<String, String>){
    let mønster = Regex::new(r"GET /(\w*) HTTP/1.1").unwrap();
    let fanget = mønster.captures(&spørrelinje).unwrap();
    let fanget = &fanget[1];
    
    let (status, filnamn) = if !sidar.contains_key(fanget){
        ("HTTP/1.1 404 NOT FOUND".to_string(), "404.html".to_string())
    }
    else{
        let side = sidar[fanget].clone();
        ("HTTP/1.1 200 OK".to_string(), side)
    };

    let innhald = fs::read_to_string(filnamn).unwrap();
    let svar = lag_svar(&status, innhald);
    
    straum.write_all(svar.as_bytes()).unwrap();
}

fn lag_svar(status: &str, innhald: String) -> String{
    let lengde = innhald.len();
    format!("{status}\r\nContent-Length: {lengde}\r\n\r\n{innhald}")
}

fn hent_html() -> io::Result<HashMap<String, String>>{
    let mut resultat = HashMap::new();
    
    let mønster = Regex::new(r"./(\w+).html").unwrap();

    for sti in fs::read_dir("./")?{
        let sti = sti?.path();
        if let Some("html") = sti.extension().and_then(OsStr::to_str){
            let sti = sti.to_owned().into_os_string().into_string().unwrap();
            if sti.contains(&"indeks"){
                resultat.insert("".to_owned(), "indeks.html".to_owned());
                resultat.insert("indeks".to_owned(), "indeks.html".to_owned());
                continue;
            }
            let fanget = mønster.captures(&sti).unwrap();
            let fanget = &fanget[1];
            resultat.insert(fanget.to_string(), format!("{fanget}.html"));            
        }
    }
    Ok(resultat)
}