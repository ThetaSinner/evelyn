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

use model::{ErrorModel};

#[derive(Serialize, Deserialize)]
pub struct CreateSimpleTaskModel {
    #[serde(rename="Token")]
    pub token: String,

    #[serde(rename="Title")]
    pub title: String,

    #[serde(rename="Description")]
    pub description: String,

    #[serde(rename="DueDate")]
    pub due_date: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateSimpleTaskResponseModel {
    #[serde(rename="Error")]
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleTaskModel {
    #[serde(rename="userId")]
    pub user_id: String,

    #[serde(rename="title")]
    pub title: String,

    #[serde(rename="description")]
    pub description: String,

    #[serde(rename="dueDate")]
    pub due_date: String,
}

#[derive(Serialize, Deserialize)]
pub struct LookupSimpleTaskRequestModel {
    #[serde(rename="Token")]
    pub token: String,

    #[serde(rename="Limit")]
    pub limit: u32,
}

#[derive(Serialize, Deserialize)]
pub struct LookupSimpleTaskResponseModel {
    #[serde(rename="Error")]
    pub error: Option<ErrorModel>,

    #[serde(rename="SimpleTasks")]
    pub tasks: Vec<SimpleTaskModel>,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleTaskLookupModel {
    #[serde(rename="UserId")]
    pub user_id: String,

    #[serde(rename="Limit")]
    pub limit: u32,
}
