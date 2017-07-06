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
use data::agile::heirarchy as heirarchy_data;
use model::agile::heirarchy as heirarchy_model;
use data::user as user_data;
use processing::ProcessorData;
use std::sync::Arc;
use uuid::Uuid;
use core::date_time_service as dts;

pub fn create(
    request_model: task_model::CreateTaskRequestModel,
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Result<task_model::CreateTaskResponseModel, EvelynCoreError> {
    let task_id = format!("{}", Uuid::new_v4());

    let task_model = task_model::TaskModel {
        task_id: task_id,
        created_by_user_id: session_token_model.user_id.to_owned(),
        date_created: dts::get_timestamp(),
        modified_by_user_id: session_token_model.user_id.to_owned(),
        date_modified: dts::get_timestamp(),
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
        assignment: None,
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

    match task_data::find_task_by_id(&ds, &request_model.project_id, &request_model.task_id) {
        Ok(result) => {
            if let Some(result) = result {
                let modified_by_user = user_data::find_user_by_id(&ds, &result.modified_by_user_id);

                Ok(task_model::LookupTaskResponseModel {
                    task: Some(task_model::TaskExternalModel {
                        task_id: result.task_id,
                        project_id: result.project_id,
                        title: result.title,
                        description: result.description,
                        original_estimate: result.original_estimate,
                        date_modified: dts::timestamp_to_string(result.date_modified),
                        modified_by_user: match modified_by_user {
                            Ok(Some(e)) => {
                                Some(task_model::UserExternalModel {
                                    user_name: e.user_name,
                                    user_id: e.user_id,
                                })
                            },
                            _ => {
                                None
                            },
                        },
                        assignment: match result.assignment {
                            None => None,
                            Some(a) => {
                                let assigned_to_user = user_data::find_user_by_id(&ds, &a.assigned_to_user_id);
                                let assigned_by_user = user_data::find_user_by_id(&ds, &a.assigned_by_user_id);

                                match (assigned_to_user, assigned_by_user) {
                                    (Ok(Some(a)), Ok(Some(b))) => {
                                        Some(task_model::AssignmentExternalModel {
                                            assigned_to_user: task_model::UserExternalModel {
                                                user_name: a.user_name,
                                                user_id: a.user_id,
                                            },
                                            assigned_by_user: task_model::UserExternalModel {
                                                user_name: b.user_name,
                                                user_id: b.user_id,   
                                            },
                                        })
                                    },
                                    _ => {
                                        None
                                    },
                                }
                            }
                        }
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

pub fn lookup_backlog(
    request_model: task_model::LookupBacklogRequestModel,
    processor_data: Arc<ProcessorData>,
) -> Result<task_model::LookupBacklogResponseModel, EvelynCoreError> {
    let ds = processor_data.data_store.clone();

    match heirarchy_data::lookup_links_to_type(&ds, &request_model.project_id, &heirarchy_model::LinkToTypeNameModel::Task) {
        Ok(result) => {
            let exclude_task_ids = result.into_iter().map(|link| {
                link.link_to_id
            }).collect();

            match task_data::lookup_backlog(&ds, &request_model.project_id, &exclude_task_ids) {
                Ok(result) => {
                    let tasks = result.into_iter().map(|task| {
                        task_model::TaskPreviewExternalModel {
                            task_id: task.task_id,
                            project_id: task.project_id,
                            title: task.title,
                            assignment: match task.assignment {
                                None => None,
                                Some(a) => {
                                    let assigned_to_user = user_data::find_user_by_id(&ds, &a.assigned_to_user_id);
                                    let assigned_by_user = user_data::find_user_by_id(&ds, &a.assigned_by_user_id);

                                    match (assigned_to_user, assigned_by_user) {
                                        (Ok(Some(a)), Ok(Some(b))) => {
                                            Some(task_model::AssignmentExternalModel {
                                                assigned_to_user: task_model::UserExternalModel {
                                                    user_name: a.user_name,
                                                    user_id: a.user_id,
                                                },
                                                assigned_by_user: task_model::UserExternalModel {
                                                    user_name: b.user_name,
                                                    user_id: b.user_id,   
                                                },
                                            })
                                        },
                                        _ => {
                                            None
                                        },
                                    }
                                }
                            },
                        }
                    }).collect();

                    Ok(task_model::LookupBacklogResponseModel {
                        tasks: tasks,
                        error: None,
                    })
                },
                Err(e) => Err(EvelynCoreError::FailedToLookupBacklogAgileTasks(e)),
            }
        },
        Err(e) => Err(EvelynCoreError::FailedToLookupAgileHeirarchyLinksToType(e)),
    }
}

pub fn update(
    request_model: task_model::UpdateTaskRequestModel,
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Option<EvelynCoreError> {
    let update_model = task_model::UpdateTaskModel {
        date_modified: dts::get_timestamp(),
        modified_by_user_id: session_token_model.user_id.to_owned(),
        task_id: request_model.task_id,
        title: request_model.title,
        description: request_model.description,
        original_estimate: request_model.original_estimate,
        assignment: match request_model.assign_to_user_id {
            None => None,
            Some(a) => Some(task_model::AssignmentModel {
                assigned_to_user_id: a,
                assigned_by_user_id: session_token_model.user_id.to_owned(),
            }),
        },
    };

    let ds = processor_data.data_store.clone();

    match task_data::update(&ds, update_model) {
        None => None,
        Some(e) => Some(EvelynCoreError::FailedToUpdateAgileTask(e)),
    }
}
