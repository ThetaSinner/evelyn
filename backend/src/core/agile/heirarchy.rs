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
use data::agile::heirarchy as heirarchy_data;
use model;
use model::agile::heirarchy as heirarchy_model;
use processing::ProcessorData;
use std::sync::Arc;
use chrono::prelude::*;

fn check_link(
    link_from: heirarchy_model::LinkFromTypeNameExternalModel, 
    link_to: heirarchy_model::LinkToTypeNameExternalModel
) -> Result<(heirarchy_model::LinkFromTypeNameModel, heirarchy_model::LinkToTypeNameModel), EvelynCoreError> {
    match (link_from, link_to) {
        (heirarchy_model::LinkFromTypeNameExternalModel::Sprint, heirarchy_model::LinkToTypeNameExternalModel::Story) => {
            Ok((heirarchy_model::LinkFromTypeNameModel::Sprint, heirarchy_model::LinkToTypeNameModel::Story))
        },
        _ => {
            Err(EvelynCoreError::AgileHeirarcyInvalidLink(EvelynBaseError::NothingElse))
        },
    }
}

pub fn make_link(
    request_model: heirarchy_model::MakeLinkRequestModel,
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Result<heirarchy_model::MakeLinkResponseModel, EvelynCoreError> {
    match check_link(request_model.link_from_type_name, request_model.link_to_type_name) {
        Ok((link_from, link_to)) => {
            let link_model = heirarchy_model::LinkModel {
                created_by_user_id: session_token_model.user_id,
                date_created: format!("{}", UTC::now()),
                link_from_type_name: link_from,
                link_to_type_name: link_to,
                link_from_id: request_model.link_from_id,
                link_to_id: request_model.link_to_id,
            };

            let ds = processor_data.data_store.clone();

            match heirarchy_data::insert_link(&ds, &link_model) {
                None => Ok(heirarchy_model::MakeLinkResponseModel {
                    error: None,
                }),
                Some(e) => Err(EvelynCoreError::FailedToMakeAgileHeirarchyLink(e)),
            }
        },
        Err(e) => {
            Err(e)
        },
    }
}
