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
#[serde(rename_all = "camelCase")]
pub struct TodoListItemModel {
    pub text: String,
    pub is_done: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TodoListItemExternalModel {
    pub text: String,
    pub is_done: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddItemTodoListRequestModel {
    pub token: String,
    pub todo_list_id: String,
    pub todo_list_item: TodoListItemExternalModel,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddItemTodoListResponseModel {
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddItemTodoListModel {
    pub user_id: String,
    pub todo_list_id: String,
    pub todo_list_item: TodoListItemModel,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateItemTodoListRequestModel {
    pub token: String,
    pub todo_list_id: String,
    pub item_index: i32,
    pub is_done: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateItemTodoListResponseModel {
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTodoListItemModel {
    pub user_id: String,
    pub todo_list_id: String,
    pub item_index: i32,
    pub is_done: bool,
}
