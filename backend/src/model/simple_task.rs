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
pub struct CreateSimpleTaskModel {
    pub token: String,
    pub title: String,
    pub description: String,
    pub due_date: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSimpleTaskResponseModel {
    pub task_id: Option<String>,
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleTaskModel {
    pub user_id: String,
    pub task_id: String,
    pub title: String,
    pub description: String,
    pub due_date: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LookupSimpleTaskRequestModel {
    pub token: String,
    pub limit: u32,
    pub show_completed: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LookupSimpleTaskResponseModel {
    pub error: Option<ErrorModel>,
    pub simple_tasks: Vec<SimpleTaskModel>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SimpleTaskLookupModel {
    pub user_id: String,
    pub limit: u32,
    pub show_completed: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateSimpleTaskRequestModel {
    pub token: String,
    pub task_id: String,
    pub new_title: Option<String>,
    pub new_description: Option<String>,
    pub new_due_date: Option<String>,
    pub new_completed: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateSimpleTaskResponseModel {
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SimpleTaskUpdateModel {
    pub user_id: String,
    pub task_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub due_date: Option<String>,
    pub completed: Option<bool>,
}
