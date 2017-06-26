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

pub mod item;

use model::ErrorModel;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TodoListModel {
    pub user_id: String,
    pub todo_list_id: String,
    pub title: String,
    pub todo_list_items: Vec<item::TodoListItemModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TodoListExternalModel {
    pub title: String,
    pub todo_list_items: Vec<item::TodoListItemExternalModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TodoListsModel {
    pub title: String,
    pub todo_list_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TodoListsExternalModel {
    pub title: String,
    pub todo_list_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CreateTodoListRequestModel {
    pub token: String,
    pub title: String,
    pub todo_list_items: Option<Vec<item::TodoListItemExternalModel>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CreateTodoListResponseModel {
    pub todo_list_id: Option<String>,
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LookupTodoListsRequestModel {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LookupTodoListsResponseModel {
    pub todo_lists: Option<Vec<TodoListsExternalModel>>,
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LookupTodoListsModel {
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LookupTodoListRequestModel {
    pub token: String,
    pub todo_list_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LookupTodoListModel {
    pub user_id: String,
    pub todo_list_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LookupTodoListResponseModel {
    pub todo_list: Option<TodoListExternalModel>,
    pub error: Option<ErrorModel>,
}
