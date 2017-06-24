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

use bson;
use bson::{Bson, Document};
use core::error_messages::{EvelynBaseError, EvelynDatabaseError};
use model;
use mongodb::{Client, ThreadedClient};
use mongodb::coll::options::FindOptions;
use mongodb::db::ThreadedDatabase;

pub fn insert_todo_list(
    client: &Client,
    create_todo_list_model: &model::todo_list::TodoListModel,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("todolist");

    let bson_todo_list_model = bson::to_bson(&create_todo_list_model).unwrap();

    if let bson::Bson::Document(document) = bson_todo_list_model {
        match collection.insert_one(document, None) {
            Ok(_) => None,
            Err(e) => Some(EvelynDatabaseError::InsertTodoList(e)),
        }
    } else {
        Some(EvelynDatabaseError::SerialisationFailed(EvelynBaseError::NothingElse))
    }
}

pub fn add_item_to_todo_list(
    client: &Client,
    add_item_todo_list_model: &model::todo_list::item::AddItemTodoListModel,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("todolist");

    let ref user_id = add_item_todo_list_model.user_id;
    let ref todo_list_id = add_item_todo_list_model.todo_list_id;
    let filter = doc!("userId" => user_id, "todoListId" => todo_list_id);

    let mut update_query = Document::new();
    let bson_todo_list_item_model = bson::to_bson(&add_item_todo_list_model.todo_list_item).unwrap();
    if let bson::Bson::Document(document) = bson_todo_list_item_model {
        update_query.insert("todoListItems", document);

        let mut push_update_query = Document::new();
        push_update_query.insert("$push", update_query);

        match collection.update_one(filter, push_update_query, None) {
            Ok(_) => None,
            Err(e) => Some(EvelynDatabaseError::AddItemToTodoList(e)),
        }
    } else {
        Some(EvelynDatabaseError::SerialisationFailed(EvelynBaseError::NothingElse))
    }
}

pub fn lookup_todo_lists(
    client: &Client,
    lookup_todo_lists_model: &model::todo_list::LookupTodoListsModel,
) -> Result<Vec<model::todo_list::TodoListsModel>, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("todolist");

    let ref user_id = lookup_todo_lists_model.user_id;
    let query = doc!{"userId" => user_id};

    let mut find_options = FindOptions::new();

    let mut projection = Document::new();
    projection.insert("title", Bson::I32(1));
    projection.insert("todoListId", Bson::I32(1));
    projection.insert("_id", Bson::I32(0));
    find_options.projection = Some(projection);

    let cursor = collection.find(Some(query), Some(find_options));

    match cursor {
        Ok(cursor) => {
            Ok(cursor.filter_map(|x| {
                match x {
                    Ok(x) => {
                        match bson::from_bson(bson::Bson::Document(x)) {
                            Ok(obj) => obj,
                            Err(e) => {
                                error!("BSON Serialize error in lookup todo lists {}", e);
                                None        
                            }
                        }
                    },
                    Err(e) => {
                        error!("Database error in lookup todo lists {}", e);
                        None
                    },
                }
            })
                   .collect())
        },
        Err(e) => Err(EvelynDatabaseError::LookupTodoLists(e)),
    }
}

pub fn lookup_todo_list(
    client: &Client,
    lookup_todo_list_model: &model::todo_list::LookupTodoListModel,
) -> Result<model::todo_list::TodoListModel, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("todolist");

    let ref user_id = lookup_todo_list_model.user_id;
    let ref todo_list_id = lookup_todo_list_model.todo_list_id;
    let query = doc!{"userId" => user_id, "todoListId" => todo_list_id};

    match collection.find_one(Some(query), None) {
        Ok(result) => {
            if let Some(result) = result {
                match bson::from_bson(bson::Bson::Document(result)) {
                    Ok(obj) => Ok(obj),
                    Err(e) => Err(EvelynDatabaseError::BSONDecodeFailed(e))
                }
            } else {
                Err(EvelynDatabaseError::TodoListNotFound(EvelynBaseError::NothingElse))
            }
        },
        Err(e) => Err(EvelynDatabaseError::LookupTodoList(e)),
    }
}

pub fn update_todo_list_item(
    client: &Client,
    update_todo_list_item: &model::todo_list::item::UpdateTodoListItemModel,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("todolist");

    let ref user_id = update_todo_list_item.user_id;
    let ref todo_list_id = update_todo_list_item.todo_list_id;
    let match_query = doc!{"userId" => user_id, "todoListId" => todo_list_id};

    let mut update_query = Document::new();
    update_query.insert(format!("todoListItems.{}.isDone", update_todo_list_item.item_index),
                        update_todo_list_item.is_done);

    let mut set_update_query = Document::new();
    set_update_query.insert("$set", update_query);

    match collection.update_one(match_query, set_update_query, None) {
        Ok(_) => None,
        Err(e) => Some(EvelynDatabaseError::UpdateTodoListItem(e)),
    }
}
