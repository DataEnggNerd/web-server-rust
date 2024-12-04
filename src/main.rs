use std::{
    fs,
    time::Duration,
    thread::sleep,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader}
};


fn handle_request(mut stream: TcpStream){
    let buff_reader = BufReader::new(&stream);
    
    let request_line = buff_reader.lines().next().unwrap().unwrap();

    // to simulate scenario of accepting multi requests at same time
    // every other request will stall at browser
    // sleep(Duration::from_secs(15));

    // let (status_line, file_name) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP 200 OK\r\n\r\n", "pages/200.html")
    // } else {
    //     ("HTTP 404 NOT FOUND\r\n\r\n", "pages/404.html")
    // };

    // above if else block refactored to `match` statement as there are more than two scenarios
    // no if..elif..else block in rust or not recommended

    let (status_line, file_name) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP 200 OK/r/n/r/n", "pages/200.html"),
        "GET /sleep HTTP/1.1" => {
            sleep(Duration::from_secs(5));
            ("HTTP 200 OK/r/n/r/n", "pages/200.html")
        },
        _ => ("HTTP 404 NOT FOUND/r/n/r/n", "pages/404.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len(); 
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}


fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        println!("Connection established!");
        handle_request(stream);
    }
}