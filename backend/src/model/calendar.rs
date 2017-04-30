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

use model::ErrorModel;

#[derive(Serialize, Deserialize)]
pub struct CalendarAddEventRequestModel {
    #[serde(rename="Token")]
    pub token: String,

    #[serde(rename="EventBegin")]
    pub event_begin: String,

    #[serde(rename="EventEnd")]
    pub event_end: String,

    #[serde(rename="Title")]
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct CalendarAddEventResponseModel {
    #[serde(rename="Error")]
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize)]
pub struct CalendarEventModel {
    #[serde(rename="userId")]
    pub user_id: String,

    #[serde(rename="eventId")]
    pub event_id: String,

    #[serde(rename="eventBegin")]
    pub event_begin: String,

    #[serde(rename="eventEnd")]
    pub event_end: String,

    #[serde(rename="title")]
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct CalendarLookupRequestModel {
    #[serde(rename="Token")]
    pub token: String,

    #[serde(rename="TimeRangeBegin")]
    pub time_range_begin: String,

    #[serde(rename="TimeRangeEnd")]
    pub time_range_end: String,
}

#[derive(Serialize, Deserialize)]
pub struct CalendarLookupResponseModel {
    #[serde(rename="Error")]
    pub error: ErrorModel,
}
