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

use core::error_messages::{EvelynBaseError, EvelynServiceError};
use core::user_group;
use model;
use model::user_group as user_group_model;
use processing;
use serde_json;
use server::routing::{RouterInput, RouterOutput};
use std::sync::Arc;

pub fn create_user_group_processor(
    router_input: RouterInput,
    processor_data: Arc<processing::ProcessorData>,
) -> RouterOutput {
    match decode_router_input_to_model!(user_group_model::CreateUserGroupRequestModel, router_input) {
        Ok(request_model) => {
            let session_token_model = validate_session!(processor_data, request_model);

            match user_group::create_user_group(request_model, session_token_model, processor_data) {
                Ok(response) => model_to_router_output!(response),
                Err(e) => {
                    model_to_router_output!(model::user_group::CreateUserGroupResponseModel {
                                                user_group_id: None,
                                                error: service_error_to_model!(EvelynServiceError::CreateUserGroup(e)),
                                            })
                },
            }
        },
        Err(e) => {
            model_to_router_output!(model::user_group::CreateUserGroupResponseModel {
                                        user_group_id: None,
                                        error: service_error_to_model!(EvelynServiceError::CouldNotDecodeTheRequestPayload(e)),
                                    })
        },
    }
}

pub fn lookup_user_groups_processor(
    router_input: RouterInput,
    processor_data: Arc<processing::ProcessorData>,
) -> RouterOutput {
    match decode_router_input_to_model!(user_group_model::LookupUserGroupsRequestModel, router_input) {
        Ok(request_model) => {
            let session_token_model = validate_session!(processor_data, request_model);

            match user_group::lookup_user_groups(&session_token_model, processor_data) {
                Ok(response) => model_to_router_output!(response),
                Err(e) => {
                    model_to_router_output!(model::user_group::LookupUserGroupsResponseModel {
                                                user_groups: Vec::new(),
                                                error: service_error_to_model!(EvelynServiceError::LookupUserGroups(e)),
                                            })
                },
            }
        },
        Err(e) => {
            model_to_router_output!(model::user_group::LookupUserGroupsResponseModel {
                                        user_groups: Vec::new(),
                                        error: service_error_to_model!(EvelynServiceError::CouldNotDecodeTheRequestPayload(e)),
                                    })
        },
    }
}

pub fn lookup_user_group_processor(
    router_input: RouterInput,
    processor_data: Arc<processing::ProcessorData>,
) -> RouterOutput {
    match decode_router_input_to_model!(user_group_model::LookupUserGroupRequestModel, router_input) {
        Ok(request_model) => {
            let session_token_model = validate_session!(processor_data, request_model);

            match user_group::lookup_user_group(request_model, session_token_model, processor_data) {
                Ok(response) => {
                    model_to_router_output!(response)
                },
                Err(e) => {
                    model_to_router_output!(model::user_group::LookupUserGroupResponseModel {
                                                user_group: None,
                                                error: service_error_to_model!(EvelynServiceError::LookupUserGroup(e)),
                                            })
                },
            }
        },
        Err(e) => {
            model_to_router_output!(model::user_group::LookupUserGroupResponseModel {
                                        user_group: None,
                                        error: service_error_to_model!(EvelynServiceError::CouldNotDecodeTheRequestPayload(e)),
                                    })
        },
    }
}

pub fn add_member_processor(
    router_input: RouterInput,
    processor_data: Arc<processing::ProcessorData>,
) -> RouterOutput {
    match decode_router_input_to_model!(user_group_model::member::AddMemberRequestModel, router_input) {
        Ok(request_model) => {
            validate_session!(processor_data, request_model);

            match user_group::add_member(request_model, processor_data) {
                None => model_to_router_output!(model::user_group::member::AddMemberResponseModel {
                    error: None,
                }),
                Some(e) => {
                    model_to_router_output!(model::user_group::member::AddMemberResponseModel {
                        error: service_error_to_model!(EvelynServiceError::AddMemberToUserGroup(e)),
                    })
                },
            }
        },
        Err(e) => {
            model_to_router_output!(model::user_group::member::AddMemberResponseModel {
                error: service_error_to_model!(EvelynServiceError::CouldNotDecodeTheRequestPayload(e)),
            })
        },
    }
}
