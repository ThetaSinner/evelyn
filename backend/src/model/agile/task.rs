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
pub struct CreateTaskRequestModel {
    pub token: String,
    pub project_id: String,
    pub title: String,
    pub description: Option<String>,
    pub original_estimate: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CreateTaskResponseModel {
    pub error: Option<ErrorModel>,
    pub task_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskModel {
    pub task_id: String,
    pub created_by_user_id: String,
    pub date_created: i64,
    pub modified_by_user_id: String,
    pub date_modified: i64,
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub original_estimate: String,
    pub assignment: Option<AssignmentModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LookupTaskRequestModel {
    pub token: String,
    pub project_id: String,
    pub task_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LookupTaskResponseModel {
    pub error: Option<ErrorModel>,
    pub task: Option<TaskExternalModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TaskExternalModel {
    pub task_id: String,
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub original_estimate: String,
    pub modified_by_user: Option<UserExternalModel>,
    pub date_modified: String,
    pub assignment: Option<AssignmentExternalOutputModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TaskPreviewExternalModel {
    pub task_id: String,
    pub project_id: String,
    pub title: String,
    pub assignment: Option<AssignmentExternalOutputModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UserExternalModel {
    pub user_name: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AssignmentExternalInputModel {
    pub assigned_to_user_id: String,
    pub assigned_by_user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AssignmentExternalOutputModel {
    pub assigned_to_user: UserExternalModel,
    pub assigned_by_user: UserExternalModel,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssignmentModel {
    pub assigned_to_user_id: String,
    pub assigned_by_user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateTaskRequestModel {
    pub token: String,
    pub task_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub original_estimate: Option<String>,
    pub assignment: Option<AssignmentExternalInputModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateTaskResponseModel {
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskModel {
    pub date_modified: i64,
    pub modified_by_user_id: String,
    pub task_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub original_estimate: Option<String>,
    pub assignment: Option<AssignmentModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LookupBacklogRequestModel {
    pub token: String,
    pub project_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LookupBacklogResponseModel {
    pub tasks: Vec<TaskPreviewExternalModel>,
    pub error: Option<ErrorModel>,
}
