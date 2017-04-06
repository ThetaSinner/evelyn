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

use std::error;
use std::fmt;

use serde_json;

use mongodb::error::Error as MongoDbError;

#[derive(Debug)]
pub enum EvelynServiceError {
    ReqestForActionWhichEvelynDoesNotKnowHowToDo,
    EvelynTriedToHandleTheRequestButDidNotYieldAResponse,
    CouldNotDecodeTheRequestPayload(serde_json::Error),
}

impl fmt::Display for EvelynServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Server layer.
            EvelynServiceError::ReqestForActionWhichEvelynDoesNotKnowHowToDo => {
                write!(f, "100001")
            },
            EvelynServiceError::EvelynTriedToHandleTheRequestButDidNotYieldAResponse => {
                write!(f, "100002")
            },

            // Processing layer.
            EvelynServiceError::CouldNotDecodeTheRequestPayload(_) => {
                write!(f, "100103")
            },
        }
    }
}

impl error::Error for EvelynServiceError {
    fn description(&self) -> &str {
        match *self {
            EvelynServiceError::ReqestForActionWhichEvelynDoesNotKnowHowToDo =>
                "Request for an action which Evelyn does now know how to do",
            EvelynServiceError::EvelynTriedToHandleTheRequestButDidNotYieldAResponse =>
                "Evelyn tried to handle the request but hasn't managed to give anything back",
            EvelynServiceError::CouldNotDecodeTheRequestPayload(_) =>
                "Could not decode the JSON request payload",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            EvelynServiceError::ReqestForActionWhichEvelynDoesNotKnowHowToDo => None,
            EvelynServiceError::EvelynTriedToHandleTheRequestButDidNotYieldAResponse => None,
            EvelynServiceError::CouldNotDecodeTheRequestPayload(ref err) => Some(err),
        }
    }
}

#[derive(Debug)]
pub enum EvelynDatabaseError {
    SerialisationFailed,
    InsertUser(MongoDbError),
}

impl fmt::Display for EvelynDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EvelynDatabaseError::SerialisationFailed =>
                write!(f, "Failed to serialise data for storage."),
            EvelynDatabaseError::InsertUser(ref e) =>
                write!(f, "Failed to create record for new user\n {}", e),
        }
    }
}

impl error::Error for EvelynDatabaseError {
    fn description(&self) -> &str {
        match *self {
            EvelynDatabaseError::SerialisationFailed =>
                "Failed to serialise data for storage.",
            EvelynDatabaseError::InsertUser(_) =>
                "Failed to create record for new user",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            EvelynDatabaseError::SerialisationFailed => None,
            EvelynDatabaseError::InsertUser(ref err) => Some(err),
        }
    }
}
