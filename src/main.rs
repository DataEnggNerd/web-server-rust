use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader}
};

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        println!("Connection established!");
        handle_request(stream);
    }
}

fn handle_request(mut stream: TcpStream){
    let buff_reader = BufReader::new(&stream);
    // let http_request: Vec<_> = buff_reader
    // .lines()
    // .map(|result| result.unwrap())
    // .take_while(|line| !line.is_empty())
    // .collect();
    let request_line = buff_reader.lines().next().unwrap().unwrap();

    // let mut response: String = String::new();

    // if request_line == "GET / HTTP/1.1"{
    //     // println!("Request :{http_request:#?}");
    //     let status_line = "HTTP 200 OK\r\n\r\n";
    //     let contents = fs::read_to_string("hello.html").unwrap();
    //     let length = contents.len();

    //     response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    // } else {
    //     let status_line: &str = "HTTP 404 NOT FOUND\r\n\r";
    //     let contents = fs::read_to_string("404.html").unwrap();
    //     let length = contents.len();
    //     response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    // }

    
    // let status_line = if request_line == "GET / HTTP/1.1" {"HTTP 200 OK\r\n\r\n"} else {"HTTP 404 NOT FOUND\r\n\r\n"};
    // let file_name = if request_line == "GET / HTTP/1.1" {"hello.html"} else {"404.html"};

    let (status_line, file_name) = if request_line == "GET / HTTP/1.1" {
        ("HTTP 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len(); 
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}