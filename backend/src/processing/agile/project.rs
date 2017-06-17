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

use core::error_messages::{EvelynServiceError, EvelynBaseError};
use model;
use model::agile::project as project_model;
use core::agile::project;
use processing;
use serde_json;
use server::routing::{RouterInput, RouterOutput};
use std::sync::Arc;

pub fn create_processor(
    router_input: RouterInput,
    processor_data: Arc<processing::ProcessorData>,
) -> RouterOutput {
    match decode_router_input_to_model!(project_model::CreateProjectRequestModel, router_input) {
        Ok(request_model) => {
            let session_token_model = validate_session!(processor_data, request_model);

            match project::create(request_model, session_token_model, processor_data) {
                Ok(response) => {
                    model_to_router_output!(response)
                },
                Err(e) => {
                    model_to_router_output!(project_model::CreateProjectResponseModel {
                        project_id: None,
                        error: service_error_to_model!(EvelynServiceError::CreateAgileProject(e)),
                    })
                },
            }
        },
        Err(e) => {
            model_to_router_output!(project_model::CreateProjectResponseModel {
                project_id: None,
                error: service_error_to_model!(EvelynServiceError::CouldNotDecodeTheRequestPayload(e)),
            })
        },
    }
}

pub fn add_user_contributor_processor(
    router_input: RouterInput,
    processor_data: Arc<processing::ProcessorData>,
) -> RouterOutput {
    match decode_router_input_to_model!(project_model::AddUserContributorRequestModel, router_input) {
        Ok(request_model) => {
            validate_session!(processor_data, request_model);

            match project::add_user_contributor(request_model, processor_data) {
                None => {
                    model_to_router_output!(project_model::AddUserContributorResponseModel {
                        error: None,
                    })
                },
                Some(e) => {
                    model_to_router_output!(project_model::AddUserContributorResponseModel {
                        error: service_error_to_model!(EvelynServiceError::AddUserContributorToAgileProject(e)),
                    })
                },
            }
        },
        Err(e) => {
            model_to_router_output!(project_model::AddUserContributorResponseModel {
                error: service_error_to_model!(EvelynServiceError::CouldNotDecodeTheRequestPayload(e)),
            })
        },
    }
}

pub fn add_user_group_contributor_processor(
    router_input: RouterInput,
    processor_data: Arc<processing::ProcessorData>,
) -> RouterOutput {
    match decode_router_input_to_model!(project_model::AddUserGroupContributorRequestModel, router_input) {
        Ok(request_model) => {
            validate_session!(processor_data, request_model);

            match project::add_user_group_contributor(request_model, processor_data) {
                None => {
                    model_to_router_output!(project_model::AddUserGroupContributorResponseModel {
                        error: None,
                    })
                },
                Some(e) => {
                    model_to_router_output!(project_model::AddUserGroupContributorResponseModel {
                        error: service_error_to_model!(EvelynServiceError::AddUserGroupContributorToAgileProject(e)),
                    })
                },
            }
        },
        Err(e) => {
            model_to_router_output!(project_model::AddUserGroupContributorResponseModel {
                error: service_error_to_model!(EvelynServiceError::CouldNotDecodeTheRequestPayload(e)),
            })
        },
    }
}

pub fn lookup_projects_processor(
    router_input: RouterInput,
    processor_data: Arc<processing::ProcessorData>,
) -> RouterOutput {
    match decode_router_input_to_model!(project_model::LookupProjectsRequestModel, router_input) {
        Ok(request_model) => {
            let session_token_model = validate_session!(processor_data, request_model);

            match project::lookup_projects(session_token_model, processor_data) {
                Ok(result) => {
                    model_to_router_output!(result)
                },
                Err(e) => {
                    model_to_router_output!(project_model::LookupProjectsResponseModel {
                        projects: Vec::new(),
                        error: service_error_to_model!(EvelynServiceError::LookupAgileProjects(e)),
                    })
                },
            }
        },
        Err(e) => {
            model_to_router_output!(project_model::LookupProjectsResponseModel {
                projects: Vec::new(),
                error: service_error_to_model!(EvelynServiceError::CouldNotDecodeTheRequestPayload(e)),
            })
        },
    }
}

pub fn lookup_processor(
    router_input: RouterInput,
    processor_data: Arc<processing::ProcessorData>,
) -> RouterOutput {
    match decode_router_input_to_model!(project_model::LookupRequestModel, router_input) {
        Ok(request_model) => {
            let session_token_model = validate_session!(processor_data, request_model);

            match project::lookup(request_model, session_token_model, processor_data) {
                Ok(result) => {
                    model_to_router_output!(result)
                },
                Err(e) => {
                    model_to_router_output!(project_model::LookupResponseModel {
                        project: None,
                        error: service_error_to_model!(EvelynServiceError::LookupAgileProject(e)),
                    })
                },
            }
        },
        Err(e) => {
            model_to_router_output!(project_model::LookupResponseModel {
                project: None,
                error: service_error_to_model!(EvelynServiceError::CouldNotDecodeTheRequestPayload(e)),
            })
        },
    }
}
