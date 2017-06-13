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
use data;
use model;
use processing::ProcessorData;
use std::sync::Arc;
use uuid::Uuid;

pub fn create_user_group(
    model: model::user_group::CreateUserGroupRequestModel,
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Result<model::user_group::CreateUserGroupResponseModel, EvelynCoreError> {
    let user_group_id = Uuid::new_v4();

    let user_group_model = model::user_group::UserGroupModel {
        created_by_user_id: session_token_model.user_id,
        user_group_id: format!("{}", user_group_id),
        name: model.name,
        description: model.description,
        members: Vec::new(),
    };

    let data_store = processor_data.data_store.clone();

    let error = data::user_group::insert_user_group(&data_store, &user_group_model);
    if let Some(e) = error {
        Err(EvelynCoreError::FailedToCreateUserGroup(e))
    } else {
        Ok(model::user_group::CreateUserGroupResponseModel {
               user_group_id: Some(format!("{}", user_group_id)),
               error: None,
           })
    }
}

pub fn lookup_user_groups(
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Result<model::user_group::LookupUserGroupsResponseModel, EvelynCoreError> {
    let data_store = processor_data.data_store.clone();

    match data::user_group::lookup_user_groups(session_token_model.user_id, &data_store) {
        Ok(result) => {
            let user_groups = result
                .into_iter()
                .map(|x| {
                         model::user_group::UserGroupsExternalModel {
                             user_group_id: x.user_group_id,
                             name: x.name,
                             description: x.description,
                         }
                     })
                .collect();

            Ok(model::user_group::LookupUserGroupsResponseModel {
                   user_groups: user_groups,
                   error: None,
               })
        },
        Err(e) => Err(EvelynCoreError::FailedToLookupUserGroups(e)),
    }
}

pub fn lookup_user_group(
    model: model::user_group::LookupUserGroupRequestModel,
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Result<model::user_group::LookupUserGroupResponseModel, EvelynCoreError> {
    let data_store = processor_data.data_store.clone();

    match data::user_group::lookup_user_group(&data_store, session_token_model.user_id, model.user_group_id) {
        Ok(result) => {
            Ok(model::user_group::LookupUserGroupResponseModel {
                   user_group: Some(model::user_group::UserGroupExternalModel {
                                        name: result.name,
                                        description: result.description,
                                        members: result
                                            .members
                                            .into_iter()
                                            .map(|x| {
                                                     model::user_group::member::UserGroupMemberExternalModel {
                                                         user_id: x.user_id,
                                                     }
                                                 })
                                            .collect(),
                                    }),
                   error: None,
               })
        },
        Err(e) => Err(EvelynCoreError::FailedToLookupUserGroup(e)),
    }
}

pub fn add_member(
    model: model::user_group::member::AddMemberRequestModel,
    processor_data: Arc<ProcessorData>,
) -> Option<EvelynCoreError> {
    let data_store = processor_data.data_store.clone();

    let add_member_model = model::user_group::member::AddMemberModel {
        user_group_id: model.user_group_id,
        user_group_member_model: model::user_group::member::UserGroupMemberModel {
            user_id: model.member.user_id
        }
    };

    match data::user_group::add_member(&data_store, add_member_model) {
        None => None,
        Some(e) => Some(EvelynCoreError::FailedToAddMemberToUserGroup(e)),
    }
}
