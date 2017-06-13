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

use core::token_service::TokenService;
use data::conf;
use mongodb::Client;
use server::routing::Router;

#[macro_use]
mod macros;

pub mod user;
pub mod user_group;
pub mod simple_task;
pub mod todo_list;
pub mod calendar;
pub mod server_admin;

pub struct ProcessorData {
    pub data_store: Client,
    pub token_service: TokenService,
    pub conf: conf::Conf,
    pub server_session_token: String,
}

pub fn load_processors(router: &mut Router) {
    // TODO only loaded if a flag is passed to evelyn, or the request is
    // authenticated?
    router.add_rule("/purge", server_admin::purge_processor);

    router.add_rule("/user/create", user::create_user_processor);
    router.add_rule("/user/logon", user::logon_user_processor);
    router.add_rule("/user/search", user::search_processor);

    router.add_rule("/usergroup/create", user_group::create_user_group_processor);
    router.add_rule("/usergroup/lookupgroups",
                    user_group::lookup_user_groups_processor);
    router.add_rule("/usergroup/lookupgroup",
                    user_group::lookup_user_group_processor);
    router.add_rule("/usergroup/addmember",
                    user_group::add_member_processor);

    router.add_rule("/simpletask/create",
                    simple_task::create_simple_task_processor);
    router.add_rule("/simpletask/lookup",
                    simple_task::lookup_simple_task_processor);
    router.add_rule("/simpletask/update",
                    simple_task::update_simple_task_processor);

    router.add_rule("/todolist/create", todo_list::create_todo_list_processor);
    router.add_rule("/todolist/additem", todo_list::add_item_todo_list_processor);
    router.add_rule("/todolist/lookuplists",
                    todo_list::lookup_todo_lists_processor);
    router.add_rule("/todolist/lookuplist",
                    todo_list::lookup_todo_list_processor);
    router.add_rule("/todolist/updateitem",
                    todo_list::update_item_todo_list_processor);

    router.add_rule("/calendar/addevent", calendar::calendar_add_event_processor);
}
