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
use model::agile::story as story_model;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

pub fn insert_story(
    client: &Client,
    story_model: &story_model::StoryModel,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_story");

    insert_model!(
        collection,
        story_model,
        EvelynDatabaseError::InsertAgileStory
    )
}

pub fn lookup_story(
    client: &Client,
    project_id: &String,
    story_id: &String,
) -> Result<Option<story_model::StoryModel>, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_story");

    let query = doc!{"projectId" => project_id, "storyId" => story_id};

    match collection.find_one(Some(query), None) {
        Ok(result) => {
            if result.is_some() {
                Ok(bson::from_bson(bson::Bson::Document(result.unwrap())).unwrap())
            }
            else {
                Ok(None)
            }
        },
        Err(e) => Err(EvelynDatabaseError::LookupAgileStory(e)),
    }
}


pub fn lookup_backlog(
    client: &Client,
    project_id: &String,
    exclude_story_ids: &Vec<String>,
) -> Result<Vec<story_model::StoryModel>, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_story");

    let mut bson_exclude_story_ids = bson::Array::new();
    for id in exclude_story_ids {
        bson_exclude_story_ids.push(Bson::String(id.to_owned()));
    }

    let mut not_in_exclude_story_ids_query = Document::new();
    not_in_exclude_story_ids_query.insert("$nin", bson_exclude_story_ids);

    let query = doc!{"projectId" => project_id, "storyId" => not_in_exclude_story_ids_query};

    let cursor = collection.find(Some(query), None);

    match cursor {
        Ok(c) => {
            Ok(c.map(|x| match x {
                Ok(x) => bson::from_bson(bson::Bson::Document(x)).unwrap(),
                Err(e) => {
                    println!("Database error in lookup backlog agile stories {}", e);
                    panic!()
                },
            }).collect())
        },
        Err(e) => Err(EvelynDatabaseError::LookupBacklogAgileStories(e)),
    }
}
