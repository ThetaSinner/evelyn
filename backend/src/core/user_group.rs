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

pub fn create_user_group(
    model: model::user_group::CreateUserGroupRequestModel,
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>)
    -> Result<model::user_group::CreateUserGroupResponseModel, EvelynCoreError>
{
  let user_group_id = Uuid::new_v4();

  let user_group_model = model::user_group::UserGroupModel {
    created_by_user_id: session_token_model.user_id,
    user_group_id: format!("{}", user_group_id),
    name: model.name,
    description: model.description,
    members: Vec::new()
  };

  let data_store = processor_data.data_store.clone();

  let error = data::user_group::insert_user_group(&data_store, &user_group_model);
  if let Some(e) = error {
    Err(EvelynCoreError::FailedToCreateUserGroup(e))
  }
  else {
    Ok(model::user_group::CreateUserGroupResponseModel {
        user_group_id: Some(format!("{}", user_group_id)),
        error: None
    })
  }
}
