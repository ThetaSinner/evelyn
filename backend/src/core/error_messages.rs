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

    // user
    CreateUser(EvelynCoreError),
    UserAlreadyExists(EvelynCoreError),
    LogonUser(EvelynCoreError),
    FailedToLogonUser(EvelynCoreError),
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
                write!(f, "100101")
            },

            EvelynServiceError::CreateUser(_) => {
                write!(f, "100201")
            },

            EvelynServiceError::UserAlreadyExists(_) => {
                write!(f, "100202")
            },

            EvelynServiceError::LogonUser(_) => {
                write!(f, "100203")
            }, 

            EvelynServiceError::FailedToLogonUser(_) => {
                write!(f, "100204")
            }
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
            EvelynServiceError::CreateUser(_) =>
                "Failed to create user",
            EvelynServiceError::UserAlreadyExists(_) =>
                "Failed to create user, a user with that name already exists",
            EvelynServiceError::LogonUser(_) =>
                "Invalid logon",
            EvelynServiceError::FailedToLogonUser(_) =>
                "Failed to logon user",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            EvelynServiceError::ReqestForActionWhichEvelynDoesNotKnowHowToDo => None,
            EvelynServiceError::EvelynTriedToHandleTheRequestButDidNotYieldAResponse => None,
            EvelynServiceError::CouldNotDecodeTheRequestPayload(ref err) => Some(err),
            EvelynServiceError::CreateUser(ref err) => Some(err),
            EvelynServiceError::UserAlreadyExists(ref err) => Some(err),
            EvelynServiceError::LogonUser(ref err) => Some(err),
            EvelynServiceError::FailedToLogonUser(ref err) => Some(err),
        }
    }
}

#[derive(Debug)]
pub enum EvelynCoreError {
    // user
    WillNotCreateUserBecauseUserAlreadyExists,
    CannotCheckIfUserExistsSoWillNotCreateNewUser(EvelynDatabaseError),
    FailedToCreateUser(EvelynDatabaseError),
    InvalidLogon,
    FailedToLogonUser(EvelynDatabaseError),
}

impl fmt::Display for EvelynCoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EvelynCoreError::WillNotCreateUserBecauseUserAlreadyExists =>
                write!(f, "Will not create the requested user because that user already exists."),
            EvelynCoreError::CannotCheckIfUserExistsSoWillNotCreateNewUser(ref err) =>
                write!(f, "Cannot check if the user exists so a new user will not be ceated: {}", err),
            EvelynCoreError::FailedToCreateUser(ref err) =>
                write!(f, "Failed to create user: {}", err),
            EvelynCoreError::InvalidLogon =>
                write!(f, "Invalid logon"),
            EvelynCoreError::FailedToLogonUser(ref err) =>
                write!(f, "Failed to logon user: {}", err)
        }
    }
}

impl error::Error for EvelynCoreError {
    fn description(&self) -> &str {
        match *self {
            EvelynCoreError::WillNotCreateUserBecauseUserAlreadyExists =>
                "Will not create the requested user because that user already exists.",
            EvelynCoreError::CannotCheckIfUserExistsSoWillNotCreateNewUser(_) =>
                "Cannot check if the user exists so a new user will not be ceated.",
            EvelynCoreError::FailedToCreateUser(_) =>
                "Failed to create user",
            EvelynCoreError::InvalidLogon =>
                "Invalid Logon",
            EvelynCoreError::FailedToLogonUser(_) =>
                "Failed to logon user",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            EvelynCoreError::WillNotCreateUserBecauseUserAlreadyExists => None,
            EvelynCoreError::CannotCheckIfUserExistsSoWillNotCreateNewUser(ref err) => Some(err),
            EvelynCoreError::FailedToCreateUser(ref err) => Some(err),
            EvelynCoreError::InvalidLogon => None,
            EvelynCoreError::FailedToLogonUser(ref err) => Some(err),
        }
    }
}

#[derive(Debug)]
pub enum EvelynDatabaseError {
    SerialisationFailed,
    InsertUser(MongoDbError),
    LookupUser(MongoDbError),
}

impl fmt::Display for EvelynDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EvelynDatabaseError::SerialisationFailed =>
                write!(f, "Failed to serialise data for storage."),
            EvelynDatabaseError::InsertUser(ref e) =>
                write!(f, "Failed to create record for new user\n {}", e),
            EvelynDatabaseError::LookupUser(ref e) =>
                write!(f, "Failed to lookup user: {}", e),
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
            EvelynDatabaseError::LookupUser(_) =>
                "Failed to lookup user",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            EvelynDatabaseError::SerialisationFailed => None,
            EvelynDatabaseError::InsertUser(ref err) => Some(err),
            EvelynDatabaseError::LookupUser(ref err) => Some(err),
        }
    }
}
