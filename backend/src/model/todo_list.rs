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
pub struct CreateTodoListRequestModel {
    #[serde(rename="Token")]
    pub token: String,

    #[serde(rename="Title")]
    pub title: String,

    #[serde(rename="TodoListItems")]
    pub todo_list_items: Option<Vec<TodoListItemExternalModel>>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTodoListResponseModel {
    #[serde(rename="TodoListId")]
    pub todo_list_id: Option<String>,

    #[serde(rename="Error")]
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize)]
pub struct TodoListItemExternalModel {
    #[serde(rename="Text")]
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoListItemModel {
    #[serde(rename="text")]
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoListModel {
    #[serde(rename="userId")]
    pub user_id: String,

    #[serde(rename="todoListId")]
    pub todo_list_id: String,

    #[serde(rename="title")]
    pub title: String,

    #[serde(rename="todoListItems")]
    pub todo_list_items: Vec<TodoListItemModel>,
}

#[derive(Serialize, Deserialize)]
pub struct AddItemTodoListRequestModel {
    #[serde(rename="Token")]
    pub token: String,

    #[serde(rename="TodoListId")]
    pub todo_list_id: String,

    #[serde(rename="TodoListItem")]
    pub todo_list_item: TodoListItemExternalModel,
}

#[derive(Serialize, Deserialize)]
pub struct AddItemTodoListResponseModel {
    #[serde(rename="Error")]
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize)]
pub struct AddItemTodoListModel {
    #[serde(rename="userId")]
    pub user_id: String,

    #[serde(rename="todoListId")]
    pub todo_list_id: String,

    #[serde(rename="todoListItem")]
    pub todo_list_item: TodoListItemModel,
}

#[derive(Serialize, Deserialize)]
pub struct LookupTodoListsRequestModel {
    #[serde(rename="Token")]
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoListsExternalModel {
    #[serde(rename="Title")]
    pub title: String,

    #[serde(rename="TodoListId")]
    pub todo_list_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct LookupTodoListsResponseModel {
    #[serde(rename="TodoLists")]
    pub todo_lists: Option<Vec<TodoListsExternalModel>>,

    #[serde(rename="Error")]
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize)]
pub struct LookupTodoListsModel {
    #[serde(rename="userId")]
    pub user_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoListsModel {
    #[serde(rename="title")]
    pub title: String,

    #[serde(rename="todoListId")]
    pub todo_list_id: String,
}