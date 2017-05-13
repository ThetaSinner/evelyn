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

use model::ErrorModel;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateUserGroupRequestModel {
    pub token: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateUserGroupResponseModel {
    pub user_group_id: Option<String>,
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupMemberModel {
    pub user_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupModel {
    pub user_group_id: String,
    pub created_by_user_id: String,
    pub name: String,
    pub description: String,
    pub members: Vec<UserGroupMemberModel>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LookupUserGroupsRequestModel {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserGroupsExternalModel {
    pub user_group_id: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LookupUserGroupsResponseModel {
    pub user_groups: Vec<UserGroupsExternalModel>,
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupsModel {
    pub user_group_id: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LookupUserGroupRequestModel {
    pub token: String,
    pub user_group_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LookupUserGroupResponseModel {
    pub user_group: Option<UserGroupExternalModel>,
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserGroupExternalModel {
    pub name: String,
    pub description: String,
    pub members: Vec<UserGroupMemberExternalModel>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserGroupMemberExternalModel {
    pub user_id: String,
}
