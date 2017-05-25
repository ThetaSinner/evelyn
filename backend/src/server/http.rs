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

use std::sync::Arc;
use std::io::Read;

use time;
use unicase::UniCase;
use hyper::server::{Handler, Request, Response, Server};
use hyper::method::Method;
use hyper::status::StatusCode;
use hyper::mime::{Attr, Mime, SubLevel, TopLevel, Value};
use hyper::header::{AccessControlAllowHeaders, AccessControlAllowMethods, AccessControlAllowOrigin, AccessControlMaxAge, Connection, ConnectionOption, ContentType, Date, HttpDate,
                    Server as HeaderServer};
use hyper_openssl::OpensslServer;
use serde_json;

use processing::ProcessorData;
use server::routing::{Router, RouterInput};
use model;
use core::error_messages;

pub struct HttpServer {
    router: Arc<Router>,
    processor_data: Arc<ProcessorData>,
    port: i64,
    hostname: String,
    use_ssl: bool,
}

struct HttpHandler {
    router: Arc<Router>,
    processor_data: Arc<ProcessorData>,
}

impl Handler for HttpHandler {
    fn handle(
        &self,
        mut req: Request,
        mut res: Response,
    ) {
        info!("Starting to handle http request");

        res.headers_mut().set(HeaderServer("evelyn".to_owned()));
        res.headers_mut().set(Date(HttpDate(time::now())));
        res.headers_mut().set(AccessControlAllowOrigin::Any);
        // vary
        // keep alive
        res.headers_mut()
            .set(Connection(vec![ConnectionOption::KeepAlive]));
        res.headers_mut()
            .set(ContentType(Mime(TopLevel::Application,
                                  SubLevel::Json,
                                  vec![(Attr::Charset, Value::Utf8)])));

        debug!("Process request {}", req.method);

        match req.method {
            Method::Options => {
                res.headers_mut().set(AccessControlAllowOrigin::Any);
                res.headers_mut()
                    .set(AccessControlAllowMethods(vec![Method::Post, Method::Get, Method::Options]));
                res.headers_mut()
                    .set(AccessControlAllowHeaders(vec![UniCase("Content-Type".to_owned())]));
                res.headers_mut().set(AccessControlMaxAge(86400));

                res.send(b"").unwrap();
            },
            Method::Post => {
                let mut body = String::new();
                req.read_to_string(&mut body).unwrap();

                debug!("Got body {}", body);

                let router_output = self.router
                    .clone()
                    .route(format!("{}", req.uri).as_str(),
                           RouterInput {
                               request_body: body,
                           },
                           self.processor_data.clone());

                if let Some(router_output) = router_output {
                    //res.headers_mut().set(ContentLength(router_output.response_body.len() as u64));
                    //let mut res = res.start().unwrap();
                    debug!("Server output {}", router_output.response_body);
                    res.send(router_output.response_body.as_bytes()).unwrap();
                } else {
                    *res.status_mut() = StatusCode::InternalServerError;
                    let model: model::ErrorModel = From::from(error_messages::EvelynServiceError::EvelynTriedToHandleTheRequestButDidNotYieldAResponse(error_messages::EvelynBaseError::NothingElse));
                    let response_body = serde_json::to_string(&model).unwrap();
                    res.send(response_body.as_bytes()).unwrap();
                }
            },
            _ => {
                *res.status_mut() = StatusCode::BadRequest;
                let model: model::ErrorModel = From::from(error_messages::EvelynServiceError::UnsupportedHttpMethod(error_messages::EvelynBaseError::NothingElse));
                let response_body = serde_json::to_string(&model).unwrap();
                res.send(response_body.as_bytes()).unwrap();
            },
        }
    }
}

impl HttpServer {
    pub fn new(
        router: Router,
        processor_data: ProcessorData,
    ) -> Self {
        HttpServer {
            router: Arc::new(router),
            port: processor_data.conf.get_port(),
            hostname: processor_data.conf.get_hostname(),
            use_ssl: processor_data.conf.is_use_ssl(),
            processor_data: Arc::new(processor_data),
        }
    }

    pub fn start(&self) {
        let addr = format!("{}:{}", self.hostname, self.port);

        debug!("Server starting with address {}, and ssl? {}",
               addr,
               self.use_ssl);

        if self.use_ssl {
            let open_ssl_server = OpensslServer::from_files("configs/certs/ia.key", "configs/certs/ia.crt").unwrap();

            Server::https(addr, open_ssl_server)
                .unwrap()
                .handle(HttpHandler {
                            router: self.router.clone(),
                            processor_data: self.processor_data.clone(),
                        })
                .unwrap();
        } else {
            Server::http(addr)
                .unwrap()
                .handle(HttpHandler {
                            router: self.router.clone(),
                            processor_data: self.processor_data.clone(),
                        })
                .unwrap();
        }
    }
}
