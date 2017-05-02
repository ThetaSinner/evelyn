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

use mongodb::db::ThreadedDatabase;

use core::error_messages::EvelynDatabaseError;
use mongodb::{Client, ThreadedClient};

pub fn purge_all(client : &Client) -> Option<EvelynDatabaseError> {
    let db = client.db("evelyn");

    match db.drop_database() {
      Ok(_) => None,
      Err(e) => Some(EvelynDatabaseError::PurgeAll(e))
    }
}

pub fn purge_user(client : &Client) -> Option<EvelynDatabaseError> {
    let db = client.db("evelyn");

    match db.drop_collection("user") {
      Ok(_) => None,
      Err(e) => Some(EvelynDatabaseError::PurgeUser(e))
    }
}

pub fn purge_simple_task(client : &Client) -> Option<EvelynDatabaseError> {
    let db = client.db("evelyn");

    match db.drop_collection("simple_task") {
      Ok(_) => None,
      Err(e) => Some(EvelynDatabaseError::PurgeSimpleTask(e))
    }
}

pub fn purge_todo_list(client : &Client) -> Option<EvelynDatabaseError> {
    let db = client.db("evelyn");

    match db.drop_collection("todolist") {
      Ok(_) => None,
      Err(e) => Some(EvelynDatabaseError::PurgeTodoList(e))
    }
}

pub fn purge_calendar(client : &Client) -> Option<EvelynDatabaseError> {
    let db = client.db("evelyn");

    match db.drop_collection("calendar") {
      Ok(_) => None,
      Err(e) => Some(EvelynDatabaseError::PurgeCalendar(e))
    }
}
