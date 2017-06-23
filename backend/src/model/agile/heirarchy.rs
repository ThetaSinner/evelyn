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

use model::ErrorModel;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LinkFromTypeNameExternalModel {
    Sprint,
    Story,
    Task,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LinkToTypeNameExternalModel {
    Story,
    Task,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LinkFromTypeNameModel {
    Sprint,
    Story,
    Task,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LinkToTypeNameModel {
    Story,
    Task,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MakeLinkRequestModel {
    pub token: String,
    pub link_from_type_name: LinkFromTypeNameExternalModel,
    pub link_to_type_name: LinkToTypeNameExternalModel,
    pub link_from_id: String,
    pub link_to_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MakeLinkResponseModel {
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkModel {
    pub created_by_user_id: String,
    pub date_created: String,
    pub link_from_type_name: LinkFromTypeNameModel,
    pub link_to_type_name: LinkToTypeNameModel,
    pub link_from_id: String,
    pub link_to_id: String,
}
