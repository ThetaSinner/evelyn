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

pub fn get_timestamp() -> i64 {
    Utc::now().timestamp()
}

pub fn timestamp_to_string(timestamp: i64) -> String {
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc).to_rfc3339()
}
