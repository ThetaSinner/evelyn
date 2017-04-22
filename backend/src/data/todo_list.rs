use bson;
use bson::{Bson, Document};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::options::FindOptions;

use model;
use core::error_messages::EvelynDatabaseError;
use mongodb::{Client, ThreadedClient};

pub fn insert_todo_list(client : &Client, create_todo_list_model: &model::todo_list::TodoListModel) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("todolist");

    let bson_todo_list_model = bson::to_bson(&create_todo_list_model).unwrap();

    if let bson::Bson::Document(document) = bson_todo_list_model {
      match collection.insert_one(document, None) {
          Ok(_) => None,
          Err(e) => Some(EvelynDatabaseError::InsertTodoList(e))
      }
    }
    else {
      Some(EvelynDatabaseError::SerialisationFailed)
    }
}

pub fn add_item_to_todo_list(client : &Client, add_item_todo_list_model: &model::todo_list::AddItemTodoListModel) -> Option<EvelynDatabaseError> {
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
            Err(e) => {
                Some(EvelynDatabaseError::AddItemToTodoList(e))
            }
        }
    }
    else {
        Some(EvelynDatabaseError::SerialisationFailed)
    }
}

pub fn lookup_todo_lists(client : &Client, lookup_todo_lists_model: &model::todo_list::LookupTodoListsModel) -> Result<Vec<model::todo_list::TodoListsModel>, EvelynDatabaseError> {
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
            Ok(cursor.map(|x| {
                match x {
                    Ok(x) => {
                        bson::from_bson(bson::Bson::Document(x)).unwrap()
                    },
                    Err(e) => {
                        println!("Database error in lookup todo lists {}", e);
                        panic!() // need a better way to handle this ideally.
                    }
                }
            }).collect())
        },
        Err(e) => {
            Err(EvelynDatabaseError::LookupTodoLists(e))
        }
    }
}
