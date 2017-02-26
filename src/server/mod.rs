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

use std::io::{Read, Write, BufReader, BufWriter, BufRead};
use std::net::{TcpListener, TcpStream};
use std::str;

pub fn start() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let stream = listener.accept().unwrap().0;

    read_request(stream);
}

fn read_request(stream: TcpStream) {
    let mut reader = BufReader::new(&stream);

    let mut data = reader.fill_buf().unwrap();
    println!("{:?}", str::from_utf8(data).unwrap());

    let mut writer = BufWriter::new(&stream);
    send_response(&mut writer);
}

fn send_response<W: Write>(writer: &mut BufWriter<W>) {
    // Write the header and the html body
    let response = "HTTP/1.1 200 OK\r\n\r\n<html><body>Hello, World!</body></html>";
    writer.write_all(response.as_bytes()).unwrap();
}
