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

#[macro_use(bson, doc)] extern crate bson;
#[macro_use] extern crate serde_derive;
extern crate mongodb;
extern crate serde_json;
extern crate jsonwebtoken as jwt;
extern crate rustc_serialize;
extern crate chrono;
extern crate config;
extern crate uuid;

mod server;
mod data;
mod processing;
pub mod model;
pub mod core;

use mongodb::{Client, ThreadedClient};

use processing::ProcessorData;
use server::http::HttpServer;
use server::routing::Router;

pub fn hello_evelyn() {
  println!("Starting...");

  let conf = data::conf::Conf::new();
  let uri = conf.get_db_connnection_string();
  // Note this will not fail if MongoDB is not available.
  let client = match Client::with_uri(uri.as_str()) {
      Ok(client) => client,
      Err(e) => panic!("Connection to the database failed {}", e)
  };

  let token_service = core::token_service::TokenService::new(String::from("a_very_important_secret"));

  let processor_data = ProcessorData{
      data_store: client,
      token_service: token_service,
      conf: conf,
  };

  let mut router = Router::new();
  processing::load_processors(&mut router);

  let http_server = HttpServer::new(router, processor_data);
  println!("Ready");
  http_server.start();
}
