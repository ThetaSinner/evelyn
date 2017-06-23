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
use data::agile::project as project_data;
use model;
use model::agile::project as project_model;
use core::user_group;
use data::user_group as user_group_data;
use data::user;
use processing::ProcessorData;
use std::sync::Arc;
use uuid::Uuid;
use chrono::prelude::*;

pub fn create(
    request_model: project_model::CreateProjectRequestModel,
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Result<project_model::CreateProjectResponseModel, EvelynCoreError> {
    let project_id = format!("{}", Uuid::new_v4());

    let project_model = project_model::ProjectModel {
        project_id: project_id,
        created_by_user_id: session_token_model.user_id,
        date_created: format!("{}", Utc::now()),
        name: request_model.name,
        short_name: request_model.short_name,
        description: request_model.description,
        user_contributors: Vec::new(),
        user_group_contributors: Vec::new(),
    };

    let ds = processor_data.data_store.clone();

    match project_data::insert_project(&ds, &project_model) {
        None => Ok(project_model::CreateProjectResponseModel {
            project_id: Some(project_model.project_id),
            error: None,
        }),
        Some(e) => Err(EvelynCoreError::FailedToCreateAgileProject(e)),
    }
}

pub fn add_user_contributor(
    request_model: project_model::AddUserContributorRequestModel,
    processor_data: Arc<ProcessorData>,
) -> Option<EvelynCoreError> {
    let user_contributor_model = project_model::AddUserContributorModel {
        project_id: request_model.project_id,
        user_contributor: project_model::UserContributorModel {
            user_id: request_model.user_contributor.user_id,
        },
    };

    let ds = processor_data.data_store.clone();

    match project_data::add_user_contributor(&ds, user_contributor_model) {
        None => None,
        Some(e) => Some(EvelynCoreError::FailedToAddUserContributorToAgileProject(e)),
    }
}

pub fn add_user_group_contributor(
    request_model: project_model::AddUserGroupContributorRequestModel,
    processor_data: Arc<ProcessorData>,
) -> Option<EvelynCoreError> {
    let group_contributor_model = project_model::AddUserGroupContributorModel {
        project_id: request_model.project_id,
        user_group_contributor: project_model::UserGroupContributorModel {
            user_group_id: request_model.user_group_contributor.user_group_id,
        },
    };

    let ds = processor_data.data_store.clone();

    match project_data::add_user_group_contributor(&ds, group_contributor_model) {
        None => None,
        Some(e) => Some(EvelynCoreError::FailedToAddUserGroupContributorToAgileProject(e)),
    }
}

pub fn lookup_projects(
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Result<project_model::LookupProjectsResponseModel, EvelynCoreError> {
    let user_groups_response_model = user_group::lookup_user_groups(&session_token_model, processor_data.clone()).unwrap();

    let ds = processor_data.data_store.clone();

    match project_data::lookup_projects(&ds, &session_token_model.user_id, user_groups_response_model.user_groups) {
        Ok(results) => Ok(project_model::LookupProjectsResponseModel {
            projects: results.into_iter().map(|x| {
                project_model::ProjectsExternalModel {
                    project_id: x.project_id,
                    name: x.name,
                    short_name: x.short_name,
                    description: x.description,
                }
            }).collect(),
            error: None,
        }),
        Err(e) => Err(EvelynCoreError::FailedToLookupAgileProjects(e)),
    }
}

pub fn lookup(
    request_model: project_model::LookupRequestModel,
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Result<project_model::LookupResponseModel, EvelynCoreError> {
    let user_groups_response_model = user_group::lookup_user_groups(&session_token_model, processor_data.clone()).unwrap();

    let ds = processor_data.data_store.clone();

    match project_data::lookup(&ds, &request_model.project_id, &session_token_model.user_id, user_groups_response_model.user_groups) {
        Ok(result) => Ok(project_model::LookupResponseModel {
            project: Some(project_model::ProjectExternalModel {
                project_id: result.project_id,
                name: result.name,
                short_name: result.short_name,
                description: result.description,
                user_contributors: result.user_contributors.into_iter().map(|x| {
                    match user::find_user_by_id(&ds, &x.user_id) {
                        Ok(Some(user)) => project_model::UserContributorPreviewExternalModel {
                            user_id: x.user_id,
                            user_name: user.user_name,
                        },
                        _ => project_model::UserContributorPreviewExternalModel {
                            user_id: x.user_id,
                            user_name: "User not found".to_owned(),
                        },
                    }
                }).collect(),
                user_group_contributors: result.user_group_contributors.into_iter().map(|x| {
                    match user_group_data::lookup_user_group(&ds, &session_token_model.user_id, &x.user_group_id) {
                        Ok(user) => project_model::UserGroupContributorPreviewExternalModel {
                            user_group_id: x.user_group_id,
                            name: user.name,
                            description: user.description,
                        },
                        _ => project_model::UserGroupContributorPreviewExternalModel {
                            user_group_id: x.user_group_id,
                            name: "User group not found".to_owned(),
                            description: "User group not found".to_owned(),
                        },
                    }
                }).collect(),
            }),
            error: None,
        }),
        Err(e) => Err(EvelynCoreError::FailedToLookupAgileProject(e)),
    }
}
