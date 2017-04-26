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

use std::sync::Arc;
use std::cmp::Ordering;

use chrono::prelude::*;
use uuid::Uuid;

use data;
use processing::ProcessorData;
use model;
use core::error_messages::EvelynCoreError;

pub fn create_simple_task(model: model::simple_task::CreateSimpleTaskModel, processor_data: Arc<ProcessorData>) -> Result<model::simple_task::CreateSimpleTaskResponseModel, EvelynCoreError> {
  let session_token_model = processor_data.token_service.extract_session_token(&model.token);

  let task_id = Uuid::new_v4();

  let simple_task_model = model::simple_task::SimpleTaskModel{
    user_id: session_token_model.user_id,
    task_id: format!("{}", task_id),
    title: model.title,
    description: model.description,
    due_date: model.due_date,
    completed: false,
  };

  let ds = processor_data.data_store.clone();

  match data::simple_task::insert_simple_task(&ds, &simple_task_model) {
      Some(e) => Err(EvelynCoreError::FailedToCreateSimpleTask(e)),
      None => Ok(
        model::simple_task::CreateSimpleTaskResponseModel {
            error: None,
        })
    }
}

pub fn lookup_simple_tasks(model: model::simple_task::LookupSimpleTaskRequestModel, processor_data: Arc<ProcessorData>) -> Result<model::simple_task::LookupSimpleTaskResponseModel, EvelynCoreError> {
  let session_token_model = processor_data.token_service.extract_session_token(&model.token);

  let simple_task_lookup_model = model::simple_task::SimpleTaskLookupModel {
    user_id: session_token_model.user_id,
    limit: model.limit,
    show_completed: model.show_completed,
  };

  let ds = processor_data.data_store.clone();

  match data::simple_task::lookup_simple_tasks(&ds, &simple_task_lookup_model) {
      Ok(mut tasks) => {
        tasks.sort_by(|a, b| {
            let a_date = a.due_date.parse::<DateTime<UTC>>();
            let b_date = b.due_date.parse::<DateTime<UTC>>();

            // TODO unsafe
            if a_date.unwrap().eq(&b_date.unwrap()) {
                if a.title < b.title {
                    Ordering::Less
                }
                else {
                    Ordering::Greater
                }
            }
            else if a_date.unwrap().lt(&b_date.unwrap()) {
                Ordering::Less
            }
            else {
                Ordering::Greater
            }
        });

        let mut filtered_tasks : Vec<model::simple_task::SimpleTaskModel> = Vec::new();
        for x in tasks {
            if simple_task_lookup_model.show_completed {
                filtered_tasks.push(x);
            } else if !simple_task_lookup_model.show_completed && !x.completed {
                filtered_tasks.push(x);
            }
        }

        if simple_task_lookup_model.limit > 0 {
            filtered_tasks.truncate(simple_task_lookup_model.limit as usize);
        }

        Ok(model::simple_task::LookupSimpleTaskResponseModel {
            tasks: Some(filtered_tasks),
            error: None,
        })
    }
  Err(e) => Err(EvelynCoreError::FailedToLookupSimpleTask(e))
}
}

pub fn update_simple_task(model: model::simple_task::UpdateSimpleTaskRequestModel, processor_data: Arc<ProcessorData>) -> Option<EvelynCoreError> {
    let session_token_model = processor_data.token_service.extract_session_token(&model.token);

    let simple_task_update_model = model::simple_task::SimpleTaskUpdateModel {
      user_id: session_token_model.user_id,
      task_id: model.task_id,
      title: model.new_title,
      description: model.new_description,
      due_date: model.new_due_date,
      completed: model.new_completed,
    };

    let ds = processor_data.data_store.clone();

    match data::simple_task::update_simple_task(&ds, simple_task_update_model) {
        None => None,
        Some(e) => {
            Some(EvelynCoreError::FailedToUpdateSimpleTask(e))
        }
    }
}
