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

use config;

pub struct Conf {
    internal: config::Config
}

impl Conf {
    pub fn new() -> Self {
      let mut c = config::Config::new();
      c.set_default("port", "8080").unwrap();
      c.set_default("host", "localhost").unwrap();
      c.set_default("db_connection_string", "mongodb://localhost:27017").unwrap();

      match c.merge(config::File::new("evelyn", config::FileFormat::Yaml)) {
          Ok(_) => println!("Reading config from evelyn yaml"),
          Err(e) => println!("Failed to load config {:?}", e)
      };

      Conf{internal: c}
    }

    pub fn get_port(&self) -> i64 {
        self.internal.get_int("port").unwrap()
    }

    pub fn get_hostname(&self) -> String {
        self.internal.get_str("host").unwrap()
    }

    pub fn get_db_connnection_string(&self) -> String {
        self.internal.get_str("db_connection_string").unwrap()
    }
}
