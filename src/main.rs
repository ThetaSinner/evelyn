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

extern crate evelyn;

use evelyn::hello_evelyn;

extern crate chrono;

use chrono::prelude::*;

// temp
// use std::{thread, time};

fn main() {
  hello_evelyn();

  // let utc_time = UTC::now();
  //
  // let two_seconds = time::Duration::from_millis(2000);
  // thread::sleep(two_seconds);
  //
  // let utc_time_now = UTC::now();
  //
  // if (utc_time_now > utc_time) {
  //     println!("can compare!");
  // }
  //
  // if (utc_time > utc_time_now) {
  //     println!("but it's not a good idea");
  // }
  //
  // println!("{:?}, {:?}", utc_time, utc_time_now);
}
