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
use data::agile::sprint as sprint_data;
use model;
use model::agile::sprint as sprint_model;
use core::agile::project;
use processing::ProcessorData;
use std::sync::Arc;
use uuid::Uuid;
use core::date_time_service as dts;

pub fn create(
    request_model: sprint_model::CreateSprintRequestModel,
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Result<sprint_model::CreateSprintResponseModel, EvelynCoreError> {
    let sprint_id = format!("{}", Uuid::new_v4());

    let sprint_model = sprint_model::SprintModel {
        sprint_id: sprint_id,
        created_by_user_id: session_token_model.user_id,
        date_created: dts::get_timestamp(),
        project_id: request_model.project_id,
        title: request_model.title,
        start_date: dts::string_to_timestamp(request_model.start_date),
        end_date: dts::string_to_timestamp(request_model.end_date),
    };

    let ds = processor_data.data_store.clone();

    match sprint_data::insert_sprint(&ds, &sprint_model) {
        None => Ok(sprint_model::CreateSprintResponseModel {
            sprint_id: Some(sprint_model.sprint_id),
            error: None,
        }),
        Some(e) => Err(EvelynCoreError::FailedToCreateAgileSprint(e)),
    }
}

pub fn lookup_active(
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Result<sprint_model::LookupActiveSprintsResponseModel, EvelynCoreError> {
    let ds = processor_data.data_store.clone();

    match project::lookup_contributing_to(session_token_model, processor_data.clone()) {
        Ok(result) => {
            let project_ids = result.projects.into_iter().map(|x| {
                x.project_id
            }).collect();

            match sprint_data::find_active(&ds, &project_ids) {
                Ok(result) => Ok(sprint_model::LookupActiveSprintsResponseModel {
                    sprints: result.into_iter().map(|x| {
                        sprint_model::SprintExternalModel {
                            sprint_id: x.sprint_id,
                            project_id: x.project_id,
                            title: x.title,
                            start_date: dts::timestamp_to_string(x.start_date),
                            end_date: dts::timestamp_to_string(x.end_date),
                        }
                    }).collect(),
                    error: None,
                }),
                Err(e) => Err(EvelynCoreError::FailedToLookupActiveAgileSprints(e)),
            }
        },
        Err(e) => Err(e), // Just propogate the error from the other module.
    }    
}

pub fn lookup_backlog(
    request_model: sprint_model::LookupBacklogRequestModel,
    processor_data: Arc<ProcessorData>,
) -> Result<sprint_model::LookupBacklogResponseModel, EvelynCoreError> {
    let ds = processor_data.data_store.clone();

    match sprint_data::lookup_backlog(&ds, &request_model.project_id) {
        Ok(result) => Ok(sprint_model::LookupBacklogResponseModel {
            sprints: result.into_iter().map(|sprint| {
                sprint_model::SprintExternalModel {
                    sprint_id: sprint.sprint_id,
                    project_id: sprint.project_id,
                    title: sprint.title,
                    start_date: dts::timestamp_to_string(sprint.start_date),
                    end_date: dts::timestamp_to_string(sprint.end_date),
                }
            }).collect(),
            error: None,
        }),
        Err(e) => Err(EvelynCoreError::FailedToLookupBacklogAgileSprints(e)),
    } 
}
