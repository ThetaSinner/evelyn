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
use data::agile::task as task_data;
use model;
use model::agile::task as task_model;
use processing::ProcessorData;
use std::sync::Arc;
use uuid::Uuid;
use chrono::prelude::*;

pub fn quick_create(
    request_model: task_model::QuickCreateTaskRequestModel,
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Result<task_model::QuickCreateTaskResponseModel, EvelynCoreError> {
    let task_id = format!("{}", Uuid::new_v4());

    let task_model = task_model::TaskModel {
        task_id: task_id,
        created_by_user_id: session_token_model.user_id,
        date_created: format!("{}", UTC::now()),
        project_id: request_model.project_id,
        title: request_model.title,
        description: "".to_owned(),
        original_estimate: "0m".to_owned(),
    };

    let ds = processor_data.data_store.clone();

    match task_data::insert_task(&ds, &task_model) {
        None => Ok(task_model::QuickCreateTaskResponseModel {
            task_id: Some(task_model.task_id),
            error: None,
        }),
        Some(e) => Err(EvelynCoreError::FailedToQuickCreateAgileTask(e)),
    }
}
