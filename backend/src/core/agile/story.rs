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

use core::error_messages::{EvelynCoreError, EvelynBaseError};
use data::agile::story as story_data;
use model;
use model::agile::story as story_model;
use core::agile::heirarchy;
use model::agile::heirarchy as heirarchy_model;
use data::agile::task as task_data;
use data::agile::heirarchy as heirarchy_data;
use processing::ProcessorData;
use std::sync::Arc;
use uuid::Uuid;
use chrono::prelude::*;

fn lookup_linked_tasks(project_id: &String, story_id: &String, processor_data: Arc<ProcessorData>) -> Vec<story_model::TaskExternalModel> {
    let links = heirarchy::lookup_links(heirarchy_model::LookupLinksRequestModel {
        project_id: project_id.to_owned(),
        link_from_type_name: heirarchy_model::LinkFromTypeNameExternalModel::Story,
        link_from_id: story_id.to_owned(),
    }, processor_data.clone());

    let ds = processor_data.data_store.clone();

    match links {
        Ok(result) => {
            result.links.into_iter().filter_map(|link| {
                let task = task_data::find_task_by_id(&ds, &link.link_to_id);

                match task {
                    Ok(task) => {
                        if let Some(task) = task {
                            Some(story_model::TaskExternalModel {
                                task_id: task.task_id,
                                title: task.title,
                            })
                        }
                        else {
                            // TODO warn.
                            None
                        }
                    },
                    Err(e) => {
                        warn!("Database error while lookup up linked task for story [{}], {}", story_id, e);
                        None
                    }
                }
            }).collect()
        },
        Err(e) => {
            warn!("Error while looking up linked tasks for story [{}], {}", story_id, e);
            Vec::new()
        }
    }
}

pub fn create(
    request_model: story_model::CreateStoryRequestModel,
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Result<story_model::CreateStoryResponseModel, EvelynCoreError> {
    let story_id = format!("{}", Uuid::new_v4());

    let story_model = story_model::StoryModel {
        story_id: story_id,
        created_by_user_id: session_token_model.user_id,
        date_created: format!("{}", Utc::now()),
        project_id: request_model.project_id,
        title: request_model.title,
        description: match request_model.description {
            Some(description) => description,
            None => "".to_owned(),
        },
    };

    let ds = processor_data.data_store.clone();

    match story_data::insert_story(&ds, &story_model) {
        None => Ok(story_model::CreateStoryResponseModel {
            story_id: Some(story_model.story_id),
            error: None,
        }),
        Some(e) => Err(EvelynCoreError::FailedToCreateAgileStory(e)),
    }
}

pub fn lookup(
    request_model: story_model::LookupRequestModel,
    processor_data: Arc<ProcessorData>,
) -> Result<story_model::LookupResponseModel, EvelynCoreError> {
    let ds = processor_data.data_store.clone();

    match story_data::lookup_story(&ds, &request_model.project_id, &request_model.story_id) {
        Ok(result) => {
            if let Some(result) = result {
                Ok(story_model::LookupResponseModel {
                    story: Some(story_model::StoryExternalModel {
                        story_id: result.story_id.to_owned(),
                        project_id: result.project_id,
                        title: result.title,
                        description: result.description,
                        tasks: lookup_linked_tasks(&request_model.project_id, &result.story_id, processor_data.clone()),
                    }),
                    error: None,
                })
            }
            else {
                Err(EvelynCoreError::AgileStoryNotFound(EvelynBaseError::NothingElse))
            }
        },
        Err(e) => Err(EvelynCoreError::FailedToLookupAgileStory(e)),
    }
}

pub fn lookup_backlog(
    request_model: story_model::LookupBacklogRequestModel,
    processor_data: Arc<ProcessorData>,
) -> Result<story_model::LookupBacklogResponseModel, EvelynCoreError> {
    let ds = processor_data.data_store.clone();

    match heirarchy_data::lookup_links_to_type(&ds, &request_model.project_id, &heirarchy_model::LinkToTypeNameModel::Story) {
        Ok(result) => {
            let exclude_story_ids = result.into_iter().map(|link| {
                link.link_to_id
            }).collect();

            match story_data::lookup_backlog(&ds, &request_model.project_id, &exclude_story_ids) {
                Ok(result) => {
                    let stories = result.into_iter().map(|story| {
                        story_model::StoryExternalModel {
                            story_id: story.story_id.to_owned(),
                            project_id: story.project_id,
                            title: story.title,
                            description: story.description,
                            tasks: lookup_linked_tasks(&request_model.project_id, &story.story_id, processor_data.clone()),
                        }
                    }).collect();

                    Ok(story_model::LookupBacklogResponseModel {
                        stories: stories,
                        error: None,
                    })
                },
                Err(e) => Err(EvelynCoreError::FailedToLookupBacklogAgileStories(e)),
            }
        },
        Err(e) => Err(EvelynCoreError::FailedToLookupAgileHeirarchyLinksToType(e)),
    }
}
