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

#[macro_export]
macro_rules! insert_model {
    ($collection:expr, $model:expr, $database_error:expr) => {{
        let bson_model = bson::to_bson(&$model).unwrap();

        if let bson::Bson::Document(document) = bson_model {
          match $collection.insert_one(document, None) {
              Ok(_) => None,
              Err(e) => Some($database_error(e))
          }
        }
        else {
          Some(EvelynDatabaseError::SerialisationFailed(EvelynBaseError::NothingElse))
        }
    }}
}
