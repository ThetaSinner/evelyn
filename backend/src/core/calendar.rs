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

use uuid::Uuid;

use processing::ProcessorData;
use model;
use model::calendar as calendar_model;
use data::calendar as calendar_data;
use core::error_messages::EvelynCoreError;

pub fn calendar_add_event(
    model: calendar_model::CalendarAddEventRequestModel,
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>
)
    -> Option<EvelynCoreError>
{
  let calendar_event_model = calendar_model::CalendarEventModel {
    user_id: session_token_model.user_id,
    event_id: format!("{}", Uuid::new_v4()),
    event_begin: model.event_begin,
    event_end: model.event_end,
    title: model.title,
  };

  let ds = processor_data.data_store.clone();

  match calendar_data::insert_calendar_event(&ds, &calendar_event_model) {
      Some(e) => Some(EvelynCoreError::FailedToAddCalendarEvent(e)),
      None => None
  }
}
