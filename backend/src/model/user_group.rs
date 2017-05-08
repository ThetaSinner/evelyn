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
pub struct CreateUserGroupRequestModel {
    #[serde(rename="Token")]
    pub token: String,

    #[serde(rename="name")]
    pub name: String,

    #[serde(rename="description")]
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserGroupResponseModel {
    #[serde(rename="UserGroupId")]
    pub user_group_id: Option<String>,

    #[serde(rename="Error")]
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize)]
pub struct UserGroupMemberModel {

}

#[derive(Serialize, Deserialize)]
pub struct UserGroupModel {
    #[serde(rename="userGroupId")]
    pub user_group_id: String,

    #[serde(rename="createdByUserId")]
    pub created_by_user_id: String,

    #[serde(rename="name")]
    pub name: String,

    #[serde(rename="description")]
    pub description: String,

    #[serde(rename="members")]
    pub members: Vec<UserGroupMemberModel>,
}
