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
use model::agile::heirarchy as heirarchy_model;
use mongodb::{Client, ThreadedClient};
use mongodb::coll::options::FindOptions;
use mongodb::db::ThreadedDatabase;
use serde_json::to_string;

pub fn insert_link(
    client: &Client,
    model: &heirarchy_model::LinkModel,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_link");

    insert_model!(
        collection,
        model,
        EvelynDatabaseError::InsertAgileHeirarchyLink
    )
}

pub fn lookup_link_to(
    client: &Client,
    link_to_id: &String,
) -> Result<Vec<heirarchy_model::LinkDbIdModel>, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_link");

    let filter = doc!{"linkToId" => link_to_id};

    let mut find_options = FindOptions::new();

    let mut projection = Document::new();
    projection.insert("_id", Bson::I32(1));
    find_options.projection = Some(projection);

    let cursor = collection.find(Some(filter), Some(find_options));

    match cursor {
        Ok(cursor) => {
            Ok(cursor.filter_map(|x| {
                match x {
                    Ok(x) => {
                        match x.get("_id") {
                            Some(&Bson::ObjectId(ref id)) => Some(heirarchy_model::LinkDbIdModel {
                                _id: id.to_hex(),
                            }),
                            _ => None,
                        }
                    },
                    Err(e) => {
                        error!("Database error in lookup agile heirarchy link to {}", e);
                        None
                    },
                }
            }).collect())
        },
        Err(e) => Err(EvelynDatabaseError::LookupAgileHeirarchyLinkTo(e)),
    }
}

pub fn remove_by_db_ids(
    client: &Client,
    ids: Vec<heirarchy_model::LinkDbIdModel>,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_link");

    let mut link_ids = bson::Array::new();
    for id in ids {
        debug!("{}", id._id);
        link_ids.push(Bson::ObjectId(
            bson::oid::ObjectId::with_string(id._id.as_ref()).unwrap()
        ));
    }

    let mut link_in_filter = Document::new();
    link_in_filter.insert("$in", link_ids);

    let mut filter = Document::new();
    filter.insert("_id", link_in_filter);

    match collection.delete_many(filter, None) {
        Ok(_) => None,
        Err(e) => Some(EvelynDatabaseError::RemoveAgileHeirarchyLinksById(e)),
    }
}

pub fn lookup_links(
    client: &Client,
    link_from_type_name: &heirarchy_model::LinkFromTypeNameModel,
    link_from_id: &String,
) -> Result<Vec<heirarchy_model::LinkModel>, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_link");

    let type_name = to_string(link_from_type_name).unwrap();
    let filter = doc!{"linkFromTypeName" => type_name, "linkFromId" => link_from_id};

    let cursor = collection.find(Some(filter), None);

    match cursor {
        Ok(cursor) => {
            Ok(cursor.filter_map(|x| {
                match x {
                    Ok(x) => {
                        Some(bson::from_bson(bson::Bson::Document(x)).unwrap())
                    },
                    Err(e) => {
                        error!("Database error in lookup agile heirarchy links {}", e);
                        None
                    },
                }
            }).collect())
        },
        Err(e) => Err(EvelynDatabaseError::LookupAgileHeirarchyLinks(e)),
    }
}
