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

use core::error_messages::EvelynCoreError;
use data;
use processing::ProcessorData;
use std::sync::Arc;

pub fn purge_database(processor_data: Arc<ProcessorData>) -> Option<EvelynCoreError> {
    let ds = processor_data.data_store.clone();

    match data::server_admin::purge_database(&ds) {
        None => None,
        Some(error) => Some(EvelynCoreError::FailedToPurgeDatabase(error)),
    }
}

pub fn purge_database_area(
    target: &String,
    processor_data: Arc<ProcessorData>,
) -> Option<EvelynCoreError> {
    let ds = processor_data.data_store.clone();

    match data::server_admin::purge_collection(target, &ds) {
        None => None,
        Some(error) => Some(EvelynCoreError::FailedToPurgeDatabaseArea(error)),
    }
}
