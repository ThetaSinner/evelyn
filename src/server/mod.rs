/// Evelyn: Your personal assistant, project manager and calendar
/// Copyright (C) 2017 Gregory Jensen
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License
/// along with this program.  If not, see <http://www.gnu.org/licenses/>.

pub mod router;

use std::io::{Write, BufReader, BufWriter, BufRead};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::str::{Lines};
use std::thread;

pub fn start() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    read_request(stream);
                });
            }
            Err(e) => println!("Failed connection: {}", e)
        }
    }
}

fn read_request(stream: TcpStream) {
    let mut reader = BufReader::new(&stream);

    let data = reader.fill_buf().unwrap();
    let str_data = str::from_utf8(data).unwrap();
    println!("{:?}", str_data);
    process_request(str_data);

    let mut writer = BufWriter::new(&stream);
    send_response(&mut writer);
}

fn send_response<W: Write>(writer: &mut BufWriter<W>) {
    // Write the header and the html body
    let response = "HTTP/1.1 200 OK\r\n\r\n<html><body>Hello, World!</body></html>";
    writer.write_all(response.as_bytes()).unwrap();
}

fn process_request(request: &str) {
    let lines = request.lines();

    let mut is_processing_header = true;
    let mut header = Vec::new();
    let mut body = "".to_string();
    for line in lines {

        if line == "" {
            is_processing_header = false;
        }
        else {
            if is_processing_header {
                header.push(line);
            }
            else {
                body = format!("{}\n{}", body, line);
            }
        }
    }

    for header_line in header {
        println!("{}", header_line);
    }

    println!("\n\nbody: {}", body);
}
