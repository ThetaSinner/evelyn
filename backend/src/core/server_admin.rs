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

use std::sync::Arc;

use data;
use processing::ProcessorData;
use core::error_messages::EvelynCoreError;

pub fn purge_all(processor_data: Arc<ProcessorData>) -> Option<EvelynCoreError> {
    let ds = processor_data.data_store.clone();

    match data::server_admin::purge_all(&ds) {
        Some(error) => Some(EvelynCoreError::FailedToPurgeAll(error)),
        None => None
    }
}

pub fn purge_user(processor_data: Arc<ProcessorData>) -> Option<EvelynCoreError> {
    let ds = processor_data.data_store.clone();

    match data::server_admin::purge_user(&ds) {
        Some(error) => Some(EvelynCoreError::FailedToPurgeUser(error)),
        None => None
    }
}

pub fn purge_simple_task(processor_data: Arc<ProcessorData>) -> Option<EvelynCoreError> {
    let ds = processor_data.data_store.clone();

    match data::server_admin::purge_simple_task(&ds) {
        Some(error) => Some(EvelynCoreError::FailedToPurgeSimpleTask(error)),
        None => None
    }
}

pub fn purge_todo_list(processor_data: Arc<ProcessorData>) -> Option<EvelynCoreError> {
    let ds = processor_data.data_store.clone();

    match data::server_admin::purge_todo_list(&ds) {
        Some(error) => Some(EvelynCoreError::FailedToPurgeTodoList(error)),
        None => None
    }
}

pub fn purge_calendar(processor_data: Arc<ProcessorData>) -> Option<EvelynCoreError> {
    let ds = processor_data.data_store.clone();

    match data::server_admin::purge_calendar(&ds) {
        Some(error) => Some(EvelynCoreError::FailedToPurgeCalendar(error)),
        None => None
    }
}
