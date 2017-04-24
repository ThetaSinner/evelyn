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

use std::io::{Write, BufReader, BufWriter, BufRead};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
use std::sync::Arc;

use serde_json;

use server::routing::{Router, RouterInput};
use processing::ProcessorData;
use core::error_messages;
use model;

pub struct HttpServer {
  router: Arc<Router>,
  processor_data: Arc<ProcessorData>,
  port: i64,
  hostname: String,
}

impl HttpServer {
  pub fn new(router: Router, processor_data: ProcessorData) -> Self {
    HttpServer{
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
                  thread::spawn(|| {
                      read_request(stream, router, processor_data);
                  });
              }
              Err(e) => println!("Failed connection: {}", e)
          }
      }
  }
}

fn read_request(stream: TcpStream, router: Arc<Router>, processor_data: Arc<ProcessorData>) {
  let mut reader = BufReader::new(&stream);

  let data = reader.fill_buf().unwrap();
  let str_data = str::from_utf8(data).unwrap();
  debug!("Incoming request to evelyn rest api: {:?}", str_data);
  let process_result = process_request(str_data, router, processor_data);

  let mut writer = BufWriter::new(&stream);
  send_response(&mut writer, process_result);
}

fn send_response<W: Write>(writer: &mut BufWriter<W>, process_result: String) {
  let response = format!("{}{}{}\r\n\r\n{}",
    "HTTP/1.1 200 OK",
    "\r\nContent-Type: application/json",
    "\r\nAccess-Control-Allow-Origin: *",
    process_result);

  debug!("Outgoing response from evelyn rest api: {:?}", response);

  writer.write_all(response.as_bytes()).unwrap();
}

fn process_request(request: &str, router: Arc<Router>, processor_data: Arc<ProcessorData>) -> String {
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

  let top_line = header[0];
  let top_line_values: Vec<_> = top_line.split(' ').collect();

  let router_output = router.route(top_line_values[1], RouterInput{request_body: body}, processor_data);

  if router_output.is_some() {
     router_output.unwrap().response_body
  }
  else {
      let model: model::ErrorModel = From::from(error_messages::EvelynServiceError::EvelynTriedToHandleTheRequestButDidNotYieldAResponse);
      serde_json::to_string(&model).unwrap()
  }
}
