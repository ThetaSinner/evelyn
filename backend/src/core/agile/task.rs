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

use core::error_messages::{EvelynCoreError, EvelynBaseError};
use data::agile::task as task_data;
use model;
use model::agile::task as task_model;
use processing::ProcessorData;
use std::sync::Arc;
use uuid::Uuid;
use chrono::prelude::*;

pub fn create(
    request_model: task_model::CreateTaskRequestModel,
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Result<task_model::CreateTaskResponseModel, EvelynCoreError> {
    let task_id = format!("{}", Uuid::new_v4());

    let task_model = task_model::TaskModel {
        task_id: task_id,
        created_by_user_id: session_token_model.user_id,
        date_created: format!("{}", Utc::now()),
        project_id: request_model.project_id,
        title: request_model.title,
        description: match request_model.description {
            Some(description) => description,
            None => "".to_owned(),
        },
        original_estimate: match request_model.original_estimate {
            Some(original_estimate) => original_estimate,
            None => "0m".to_owned(),
        },
    };

    let ds = processor_data.data_store.clone();

    match task_data::insert_task(&ds, &task_model) {
        None => Ok(task_model::CreateTaskResponseModel {
            task_id: Some(task_model.task_id),
            error: None,
        }),
        Some(e) => Err(EvelynCoreError::FailedToCreateAgileTask(e)),
    }
}

pub fn lookup(
    request_model: task_model::LookupTaskRequestModel,
    processor_data: Arc<ProcessorData>,
) -> Result<task_model::LookupTaskResponseModel, EvelynCoreError> {
    let ds = processor_data.data_store.clone();

    match task_data::find_task_by_id(&ds, &request_model.task_id) {
        Ok(result) => {
            if let Some(result) = result {
                Ok(task_model::LookupTaskResponseModel {
                    task: Some(task_model::TaskExternalModel {
                        task_id: result.task_id,
                        project_id: result.project_id,
                        title: result.title,
                        description: result.description,
                        original_estimate: result.original_estimate,
                    }),
                    error: None,
                })
            }
            else {
                Err(EvelynCoreError::AgileTaskNotFound(EvelynBaseError::NothingElse))
            }
        },
        Err(e) => Err(EvelynCoreError::FailedToLookupAgileTask(e)),
    }
}
