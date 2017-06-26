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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CreateProjectRequestModel {
    pub token: String,
    pub name: String,
    pub short_name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CreateProjectResponseModel {
    pub project_id: Option<String>,
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectModel {
    pub project_id: String,
    pub created_by_user_id: String,
    pub date_created: String,
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub user_contributors: Vec<UserContributorModel>,
    pub user_group_contributors: Vec<UserGroupContributorModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserContributorModel {
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupContributorModel {
    pub user_group_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UserContributorExternalModel {
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UserGroupContributorExternalModel {
    pub user_group_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ContributorType {
    User,
    Group,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum ContributorTypeExternal {
    User,
    Group,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AddUserContributorRequestModel {
    pub token: String,
    pub project_id: String,
    pub user_contributor: UserContributorExternalModel,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AddUserContributorResponseModel {
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddUserContributorModel {
    pub project_id: String,
    pub user_contributor: UserContributorModel,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AddUserGroupContributorRequestModel {
    pub token: String,
    pub project_id: String,
    pub user_group_contributor: UserGroupContributorExternalModel,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AddUserGroupContributorResponseModel {
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddUserGroupContributorModel {
    pub project_id: String,
    pub user_group_contributor: UserGroupContributorModel,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LookupProjectsRequestModel {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LookupProjectsResponseModel {
    pub projects: Vec<ProjectsExternalModel>,
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectsModel {
    pub project_id: String,
    pub name: String,
    pub short_name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ProjectsExternalModel {
    pub project_id: String,
    pub name: String,
    pub short_name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LookupRequestModel {
    pub token: String,
    pub project_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LookupResponseModel {
    pub project: Option<ProjectExternalModel>,
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ProjectExternalModel {
    pub project_id: String,
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub user_contributors: Vec<UserContributorPreviewExternalModel>,
    pub user_group_contributors: Vec<UserGroupContributorPreviewExternalModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UserContributorPreviewExternalModel {
    pub user_id: String,
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UserGroupContributorPreviewExternalModel {
    pub user_group_id: String,
    pub name: String,
    pub description: String,
}
