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

use core::error_messages;
use model;
use processing::ProcessorData;
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;

pub struct RouterInput {
    pub request_body: String,
}

pub struct RouterOutput {
    pub response_body: String,
}

pub struct Router {
    rules: HashMap<String, fn(RouterInput, Arc<ProcessorData>) -> RouterOutput>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(
        &mut self,
        route: &str,
        processor: fn(RouterInput, Arc<ProcessorData>) -> RouterOutput,
    ) {
        self.rules.insert(route.to_string(), processor);
    }

    pub fn route(
        &self,
        route: &str,
        router_input: RouterInput,
        processor_data: Arc<ProcessorData>,
    ) -> Option<RouterOutput> {
        let processor_opt = self.rules.get(route);
        match processor_opt {
            Some(processor) => Some(processor(router_input, processor_data)),
            None => {
                let model: model::ErrorModel = From::from(error_messages::EvelynServiceError::ReqestForActionWhichEvelynDoesNotKnowHowToDo(error_messages::EvelynBaseError::NothingElse));
                Some(RouterOutput {
                         response_body: serde_json::to_string(&model).unwrap(),
                     })
            },
        }
    }
}
