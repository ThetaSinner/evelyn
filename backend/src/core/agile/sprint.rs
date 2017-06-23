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

use core::error_messages::EvelynCoreError;
use data::agile::sprint as sprint_data;
use model;
use model::agile::sprint as sprint_model;
use processing::ProcessorData;
use std::sync::Arc;
use uuid::Uuid;
use chrono::prelude::*;

pub fn create(
    request_model: sprint_model::CreateSprintRequestModel,
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Result<sprint_model::CreateSprintResponseModel, EvelynCoreError> {
    let sprint_id = format!("{}", Uuid::new_v4());

    let sprint_model = sprint_model::SprintModel {
        sprint_id: sprint_id,
        created_by_user_id: session_token_model.user_id,
        date_created: format!("{}", UTC::now()),
        title: request_model.title,
        start_date: request_model.start_date,
        end_date: request_model.end_date,
    };

    let ds = processor_data.data_store.clone();

    match sprint_data::insert_sprint(&ds, &sprint_model) {
        None => Ok(sprint_model::CreateSprintResponseModel {
            sprint_id: Some(sprint_model.sprint_id),
            error: None,
        }),
        Some(e) => Err(EvelynCoreError::FailedToCreateAgileSprint(e)),
    }
}
