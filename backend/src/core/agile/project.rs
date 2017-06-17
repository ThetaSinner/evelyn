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
use processing::ProcessorData;
use std::sync::Arc;
use uuid::Uuid;
use chrono::prelude::*;

pub fn create(
    request_model: project_model::ProjectAddRequestModel,
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Result<project_model::ProjectAddResponseModel, EvelynCoreError> {
    let project_id = format!("{}", Uuid::new_v4());

    let project_model = project_model::ProjectModel {
        project_id: project_id,
        created_by_user_id: session_token_model.user_id,
        date_created: format!("{}", UTC::now()),
        name: request_model.name,
        short_name: request_model.short_name,
        description: request_model.description,
        contributors: Vec::new(),
    };

    let ds = processor_data.data_store.clone();

    match project_data::insert_project(&ds, &project_model) {
        None => Ok(project_model::ProjectAddResponseModel {
            project_id: Some(project_model.project_id),
            error: None,
        }),
        Some(e) => Err(EvelynCoreError::FailedToCreateAgileProject(e)),
    }
}

pub fn add_contributor(
    request_model: project_model::CreateContributorRequestModel,
    processor_data: Arc<ProcessorData>,
) -> Option<EvelynCoreError> {
    let contributor_model = project_model::CreateContributorModel {
        project_id: request_model.project_id,
        contributor: project_model::ContributorModel {
            id_type: match request_model.contributor.id_type {
                project_model::IdTypeExternal::User => project_model::IdType::User,
                project_model::IdTypeExternal::Group => project_model::IdType::Group,
            },
            id: request_model.contributor.id,
        },
    };

    let ds = processor_data.data_store.clone();

    match project_data::push_contributor(&ds, contributor_model) {
        None => None,
        Some(e) => Some(EvelynCoreError::FailedToAddContributorToAgileProject(e)),
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
        Err(e) => Err(EvelynCoreError::FailedToAddContributorToAgileProject(e)),
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
                contributors: result.contributors.into_iter().map(|x| {
                    project_model::ContributorExternalModel {
                        id_type: match x.id_type {
                            project_model::IdType::User => project_model::IdTypeExternal::User,
                            project_model::IdType::Group => project_model::IdTypeExternal::Group,
                        },
                        id: x.id,
                    }
                }).collect(),
            }),
            error: None,
        }),
        Err(e) => Err(EvelynCoreError::FailedToAddContributorToAgileProject(e)),
    }
}
