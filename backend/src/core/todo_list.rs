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

use uuid::Uuid;

use data;
use model;
use processing::ProcessorData;
use core::error_messages::EvelynCoreError;

pub fn create_todo_list(model: model::todo_list::CreateTodoListRequestModel, processor_data: Arc<ProcessorData>) -> Result<model::todo_list::CreateTodoListResponseModel, EvelynCoreError> {
  let session_token_model = processor_data.token_service.extract_session_token(&model.token);

  let todo_list_id = Uuid::new_v4();

  let mut todo_list_model = model::todo_list::TodoListModel {
    user_id: session_token_model.user_id,
    todo_list_id: format!("{}", todo_list_id),
    title: model.title,
    todo_list_items: Vec::new(),
  };

  if let Some(todo_list_items) = model.todo_list_items {
      for i in todo_list_items {
          todo_list_model.todo_list_items.push(i);
      }
  }

  let data_store = processor_data.data_store.clone();

  let error = data::insert_todo_list(&data_store, &todo_list_model);
  if let Some(e) = error {
    Err(EvelynCoreError::FailedToCreateTodoList(e))
  }
  else {
    Ok(model::todo_list::CreateTodoListResponseModel {
        todo_list_id: Some(format!("{}", todo_list_id)),
        error: None,
    })
  }
}

pub fn add_item_to_todo_list(model: model::todo_list::AddItemTodoListRequestModel, processor_data: Arc<ProcessorData>) -> Option<EvelynCoreError> {
  let session_token_model = processor_data.token_service.extract_session_token(&model.token);

  let todo_list_model = model::todo_list::AddItemTodoListModel {
    user_id: session_token_model.user_id,
    todo_list_id: model.todo_list_id,
    todo_list_item: model.todo_list_item,
  };

  let data_store = processor_data.data_store.clone();

  let error = data::add_item_to_todo_list(&data_store, &todo_list_model);
  if let Some(e) = error {
    Some(EvelynCoreError::FailedToAddItemToTodoList(e))
  }
  else {
    None
  }
}
