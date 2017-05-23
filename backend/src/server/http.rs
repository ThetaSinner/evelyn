// Evelyn: Your personal assistant, project manager and calendar
// Copyright (C) 2017 Gregory Jensen
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use chrono::prelude::*;
use core::error_messages;
use model;
use processing::ProcessorData;
use serde_json;
use server::routing::{Router, RouterInput};
use std::fmt;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::sync::Arc;
use std::thread;

struct Header {
    pub method: String,
    pub route: String,
    pub http_version: String,
}

struct HttpRequest {
    pub header: Option<Header>,
    pub body: Option<String>,
}

pub struct HttpServer {
    router: Arc<Router>,
    processor_data: Arc<ProcessorData>,
    port: i64,
    hostname: String,
}

enum HttpStatus {
    Ok,
    BadRequest,
    InternalServerError,
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HttpStatus::Ok => write!(f, "200 OK"),
            HttpStatus::BadRequest => write!(f, "400 Bad Request"),
            HttpStatus::InternalServerError => write!(f, "500 Internal Server Error"),
        }
    }
}

struct HttpProcessResult {
    pub http_status: HttpStatus,

    pub response_body: String,
}

impl HttpServer {
    pub fn new(router: Router, processor_data: ProcessorData) -> Self {
        HttpServer {
            router: Arc::new(router),
            port: processor_data.conf.get_port(),
            hostname: processor_data.conf.get_hostname(),
            processor_data: Arc::new(processor_data),
        }
    }

    pub fn start(&self) {
        let addr = format!("{}:{}", self.hostname, self.port);
        let listener = TcpListener::bind(addr.as_str()).unwrap();

        for stream in listener.incoming() {
            let router = self.router.clone();
            let processor_data = self.processor_data.clone();

            match stream {
                Ok(stream) => {
                    thread::spawn(|| read_request(stream, router, processor_data));
                },
                Err(e) => println!("Failed connection: {}", e),
            }
        }
    }
}

fn read_request(stream: TcpStream, router: Arc<Router>, processor_data: Arc<ProcessorData>) {
    let mut reader = BufReader::new(&stream);

    let data = reader.fill_buf().unwrap();
    let str_data = str::from_utf8(data).unwrap();
    debug!("Incoming request to evelyn rest api: {:?}", str_data);

    let process_result = process_request(str_data);
    let mut writer = BufWriter::new(&stream);

    match process_result {
        Ok(request) => {
            let header = request.header.unwrap();
            let body = request.body.unwrap();

            if header.method == "OPTIONS" {
                send_options_response(&mut writer)
            } else {
                let router_output = router.route(header.route.as_str(),
                                                 RouterInput { request_body: body },
                                                 processor_data);

                if router_output.is_some() {
                    send_response(&mut writer,
                                  HttpProcessResult {
                                      http_status: HttpStatus::Ok,
                                      response_body: router_output.unwrap().response_body,
                                  });
                } else {
                    let model: model::ErrorModel = From::from(error_messages::EvelynServiceError::EvelynTriedToHandleTheRequestButDidNotYieldAResponse(error_messages::EvelynBaseError::NothingElse));
                    send_response(&mut writer,
                                  HttpProcessResult {
                                      http_status: HttpStatus::InternalServerError,
                                      response_body: serde_json::to_string(&model).unwrap(),
                                  });
                }
            }
        },
        Err(e) => {
            let model: model::ErrorModel = From::from(e);
            send_response(&mut writer,
                          HttpProcessResult {
                              http_status: HttpStatus::BadRequest,
                              response_body: serde_json::to_string(&model).unwrap(),
                          });
        },
    }

}

fn send_options_response<W: Write>(writer: &mut BufWriter<W>) {
    let dt = Local::now();

    let response =
        format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                "HTTP/1.1 200 OK\r\n",
                dt.format("Date: %a %e %b %Y %T UTC\r\n").to_string(), /* "Date: Mon, 01 Dec
                                                                        * 2008 01:15:39 GMT */
                "Server: Evelyn (Unix)\r\n",
                "Access-Control-Allow-Origin: *\r\n",
                "Access-Control-Allow-Methods: POST, GET, OPTIONS\r\n",
                "Access-Control-Allow-Headers: Content-Type\r\n",
                "Access-Control-Max-Age: 86400\r\n",
                "Vary: Accept-Encoding, Origin\r\n",
                "Content-Encoding: gzip\r\n",
                "Content-Length: 0\r\n",
                "Keep-Alive: timeout=2, max=100\r\n",
                "Connection: Keep-Alive\r\n",
                "Content-Type: application/json\r\n",
                "\r\n\r\n");

    debug!("Outgoing response from evelyn rest api: {:?}", response);

    writer.write_all(response.as_bytes()).unwrap();
}

fn send_response<W: Write>(writer: &mut BufWriter<W>, process_result: HttpProcessResult) {
    let response = format!("HTTP/1.1 {}{}{}\r\n\r\n{}",
                           process_result.http_status,
                           "\r\nContent-Type: application/json",
                           "\r\nAccess-Control-Allow-Origin: *",
                           process_result.response_body);

    debug!("Outgoing response from evelyn rest api: {:?}", response);

    writer.write_all(response.as_bytes()).unwrap();
}

fn process_request(request: &str) -> Result<HttpRequest, error_messages::EvelynServiceError> {
    let lines = request.lines();

    let mut is_processing_header = true;
    let mut header = Vec::new();
    let mut body = String::new();
    for line in lines {
        if line == "" {
            is_processing_header = false;
        } else {
            if is_processing_header {
                header.push(line);
            } else {
                body = format!("{}\n{}", body, line);
            }
        }
    }

    if header.len() == 0 {
        Err(error_messages::EvelynServiceError::ExpectedHeaderOnRequestButNoneWasFound(error_messages::EvelynBaseError::NothingElse))
    } else {
        let top_line = header[0];
        let top_line_values: Vec<_> = top_line.split(' ').collect();

        let header_model = Header {
            method: String::from(top_line_values[0]),
            route: String::from(top_line_values[1]),
            http_version: String::from(top_line_values[2]),
        };

        Ok(HttpRequest {
               header: Some(header_model),
               body: Some(body),
           })
    }
}
