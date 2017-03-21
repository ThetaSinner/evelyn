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

use processing::ProcessorData;
use model;

pub fn create_simple_task(model: model::CreateSimpleTaskModel, processor_data: Arc<ProcessorData>) -> model::CreateSimpleTaskResponseModel {
  let session_token_model = processor_data.token_service.extract_session_token(&model.token);

  let simple_task_model = model::SimpleTaskModel{
    user_id: session_token_model.user_id,
    title: model.title,
    description: model.description,
    due_date: model.due_date,
  };

  let ds = processor_data.data_store.clone();
  let mut data_store = ds.lock().unwrap();

  let error = data_store.insert_simple_task(&simple_task_model);
  if error.is_some() {
    model::CreateSimpleTaskResponseModel {
        error: Some(model::ErrorModel{
            error_code: "102002".to_owned(),
            error_message: "Failed to insert simple task".to_owned()
        })
    }
  }
  else {
    model::CreateSimpleTaskResponseModel {
        error: None,
    }
  }
}

pub fn lookup_simple_tasks(model: model::LookupSimpleTaskRequestModel, processor_data: Arc<ProcessorData>) -> model::LookupSimpleTaskResponseModel {
  let session_token_model = processor_data.token_service.extract_session_token(&model.token);

  let simple_task_lookup_model = model::SimpleTaskLookupModel {
    user_id: session_token_model.user_id,
  };

  let ds = processor_data.data_store.clone();
  let mut data_store = ds.lock().unwrap();

  let tasks = data_store.lookup_simple_tasks(&simple_task_lookup_model);
  if tasks.is_some() {
      model::LookupSimpleTaskResponseModel {
          tasks: tasks.unwrap(),
          error: None,
      }
  }
  else {
      model::LookupSimpleTaskResponseModel {
          error: Some(model::ErrorModel{
              error_code: "103001".to_owned(),
              error_message: "Failed to lookup simple tasks".to_owned()
          }),
          tasks: Vec::new()
      }
  }
}
