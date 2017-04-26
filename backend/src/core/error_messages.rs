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
    ExpectedHeaderOnRequestButNoneWasFound,
    CouldNotDecodeTheRequestPayload(serde_json::Error),

    // user
    CreateUser(EvelynCoreError),
    UserAlreadyExists(EvelynCoreError),
    LogonUser(EvelynCoreError),
    FailedToLogonUser(EvelynCoreError),

    // simple task
    FailedToCreateSimpleTask(EvelynCoreError),
    FailedToUpdateSimpleTask(EvelynCoreError),
    FailedToLookupSimpleTask(EvelynCoreError),

    // todo list
    CreateTodoList(EvelynCoreError),
    AddItemToTodoList(EvelynCoreError),
    LookupTodoLists(EvelynCoreError),
    LookupTodoList(EvelynCoreError),
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
            EvelynServiceError::ExpectedHeaderOnRequestButNoneWasFound => {
                write!(f, "100003")
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
            },

            //
            // Simple Task
            //
            EvelynServiceError::FailedToCreateSimpleTask(_) => {
                write!(f, "100301")
            },
            EvelynServiceError::FailedToUpdateSimpleTask(_) => {
                write!(f, "100302")
            },
            EvelynServiceError::FailedToLookupSimpleTask(_) => {
                write!(f, "100303")
            },

            //
            // Todo List
            //
            EvelynServiceError::CreateTodoList(_) => {
                write!(f, "100401")
            },
            EvelynServiceError::AddItemToTodoList(_) => {
                write!(f, "100402")
            },
            EvelynServiceError::LookupTodoLists(_) => {
                write!(f, "100403")
            },
            EvelynServiceError::LookupTodoList(_) => {
                write!(f, "100404")
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
            EvelynServiceError::ExpectedHeaderOnRequestButNoneWasFound =>
                "Expected a header with the request but didn't find a header",
            EvelynServiceError::CouldNotDecodeTheRequestPayload(_) =>
                "Could not decode the JSON request payload",

            // User
            EvelynServiceError::CreateUser(_) =>
                "Failed to create user",
            EvelynServiceError::UserAlreadyExists(_) =>
                "Failed to create user, a user with that name already exists",
            EvelynServiceError::LogonUser(_) =>
                "Invalid logon",
            EvelynServiceError::FailedToLogonUser(_) =>
                "Failed to logon user",

            // SimpleTask
            EvelynServiceError::FailedToCreateSimpleTask(_) =>
                "Failed to create simple task",
            EvelynServiceError::FailedToUpdateSimpleTask(_) =>
                "Failed to update simple task",
            EvelynServiceError::FailedToLookupSimpleTask(_) =>
                "Failed to lookup simple task",

            // Todo List
            EvelynServiceError::CreateTodoList(_) =>
                "Failed to create todo list",
            EvelynServiceError::AddItemToTodoList(_) =>
                "Failed to add item to todo list",
            EvelynServiceError::LookupTodoLists(_) =>
                "Failed to lookup todo lists",
            EvelynServiceError::LookupTodoList(_) =>
                "Failed to lookup todo list",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            EvelynServiceError::ReqestForActionWhichEvelynDoesNotKnowHowToDo => None,
            EvelynServiceError::EvelynTriedToHandleTheRequestButDidNotYieldAResponse => None,
            EvelynServiceError::ExpectedHeaderOnRequestButNoneWasFound => None,
            EvelynServiceError::CouldNotDecodeTheRequestPayload(ref err) => Some(err),

            // User
            EvelynServiceError::CreateUser(ref err) => Some(err),
            EvelynServiceError::UserAlreadyExists(ref err) => Some(err),
            EvelynServiceError::LogonUser(ref err) => Some(err),
            EvelynServiceError::FailedToLogonUser(ref err) => Some(err),

            // Simple Task
            EvelynServiceError::FailedToCreateSimpleTask(ref err) => Some(err),
            EvelynServiceError::FailedToUpdateSimpleTask(ref err) => Some(err),
            EvelynServiceError::FailedToLookupSimpleTask(ref err) => Some(err),

            // Todo List
            EvelynServiceError::CreateTodoList(ref err) => Some(err),
            EvelynServiceError::AddItemToTodoList(ref err) => Some(err),
            EvelynServiceError::LookupTodoLists(ref err) => Some(err),
            EvelynServiceError::LookupTodoList(ref err) => Some(err),
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

    // Simple Task`
    FailedToCreateSimpleTask(EvelynDatabaseError),
    FailedToUpdateSimpleTask(EvelynDatabaseError),
    FailedToLookupSimpleTask(EvelynDatabaseError),

    // Todo List
    FailedToCreateTodoList(EvelynDatabaseError),
    FailedToAddItemToTodoList(EvelynDatabaseError),
    FailedToLookupTodoLists(EvelynDatabaseError),
    FailedToLookupTodoList(EvelynDatabaseError),
}

impl fmt::Display for EvelynCoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            //User
            EvelynCoreError::WillNotCreateUserBecauseUserAlreadyExists =>
                write!(f, "Will not create the requested user because that user already exists."),
            EvelynCoreError::CannotCheckIfUserExistsSoWillNotCreateNewUser(ref err) =>
                write!(f, "Cannot check if the user exists so a new user will not be ceated: {}", err),
            EvelynCoreError::FailedToCreateUser(ref err) =>
                write!(f, "Failed to create user: {}", err),
            EvelynCoreError::InvalidLogon =>
                write!(f, "Invalid logon"),
            EvelynCoreError::FailedToLogonUser(ref err) =>
                write!(f, "Failed to logon user: {}", err),

            // Simple Task
            EvelynCoreError::FailedToCreateSimpleTask(ref err) =>
                write!(f, "Failed to create task: {}", err),
            EvelynCoreError::FailedToUpdateSimpleTask(ref err) =>
                write!(f, "Failed to update task: {}", err),
            EvelynCoreError::FailedToLookupSimpleTask(ref err) =>
                write!(f, "Failed to lookup task: {}", err),

            //Todo List
            EvelynCoreError::FailedToCreateTodoList(ref err) =>
                write!(f, "Failed to create todo list: {}", err),
            EvelynCoreError::FailedToAddItemToTodoList(ref err) =>
                write!(f, "Failed to add item to todo list: {}", err),
            EvelynCoreError::FailedToLookupTodoLists(ref err) =>
                write!(f, "Failed to lookup todo lists: {}", err),
            EvelynCoreError::FailedToLookupTodoList(ref err) =>
                write!(f, "Failed to lookup todo list: {}", err),
        }
    }
}

impl error::Error for EvelynCoreError {
    fn description(&self) -> &str {
        match *self {
            // User
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

            // Simple Task
            EvelynCoreError::FailedToCreateSimpleTask(_) =>
                "Failed to create task",
            EvelynCoreError::FailedToUpdateSimpleTask(_) =>
                "Failed to update task",
            EvelynCoreError::FailedToLookupSimpleTask(_) =>
                "Failed to lookup task",

            // Todo List
            EvelynCoreError::FailedToCreateTodoList(_) =>
                "Failed to create todo list",
            EvelynCoreError::FailedToAddItemToTodoList(_) =>
                "Failed to add item to todo list",
            EvelynCoreError::FailedToLookupTodoLists(_) =>
                "Failed to lookup todo lists",
            EvelynCoreError::FailedToLookupTodoList(_) =>
                "Failed to lookup todo list",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // user
            EvelynCoreError::WillNotCreateUserBecauseUserAlreadyExists => None,
            EvelynCoreError::CannotCheckIfUserExistsSoWillNotCreateNewUser(ref err) => Some(err),
            EvelynCoreError::FailedToCreateUser(ref err) => Some(err),
            EvelynCoreError::InvalidLogon => None,
            EvelynCoreError::FailedToLogonUser(ref err) => Some(err),

            // Simple Task
            EvelynCoreError::FailedToCreateSimpleTask(ref err) => Some(err),
            EvelynCoreError::FailedToUpdateSimpleTask(ref err) => Some(err),
            EvelynCoreError::FailedToLookupSimpleTask(ref err) => Some(err),

            // Todo List
            EvelynCoreError::FailedToCreateTodoList(ref err) => Some(err),
            EvelynCoreError::FailedToAddItemToTodoList(ref err) => Some(err),
            EvelynCoreError::FailedToLookupTodoLists(ref err) => Some(err),
            EvelynCoreError::FailedToLookupTodoList(ref err) => Some(err),
        }
    }
}

#[derive(Debug)]
pub enum EvelynDatabaseError {
    SerialisationFailed,

    // User
    InsertUser(MongoDbError),
    LookupUser(MongoDbError),

    // Simple Task
    InsertSimpleTask(MongoDbError),
    UpdateSimpleTask(MongoDbError),
    LookupSimpleTask(MongoDbError),

    // Todo List
    InsertTodoList(MongoDbError),
    AddItemToTodoList(MongoDbError),
    LookupTodoLists(MongoDbError),
    TodoListNotFound,
    LookupTodoList(MongoDbError),
}

impl fmt::Display for EvelynDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EvelynDatabaseError::SerialisationFailed =>
                write!(f, "Failed to serialise data for storage."),

            // User
            EvelynDatabaseError::InsertUser(ref e) =>
                write!(f, "Failed to create record for new user\n {}", e),
            EvelynDatabaseError::LookupUser(ref e) =>
                write!(f, "Failed to lookup user: {}", e),

            // Simple Task
            EvelynDatabaseError::InsertSimpleTask(ref e) =>
                write!(f, "Failed to create new simple task: {}", e),
            EvelynDatabaseError::UpdateSimpleTask(ref e) =>
                write!(f, "Failed to update simple task: {}", e),
            EvelynDatabaseError::LookupSimpleTask(ref e) =>
                write!(f, "Failed to lookup simple tasks: {}", e),

            // Todo List
            EvelynDatabaseError::InsertTodoList(ref e) =>
                write!(f, "Failed to insert todo list: {}", e),
            EvelynDatabaseError::AddItemToTodoList(ref e) =>
                write!(f, "Failed to add item to todo list: {}", e),
            EvelynDatabaseError::LookupTodoLists(ref e) =>
                write!(f, "Failed to lookup todo lists: {}", e),
            EvelynDatabaseError::TodoListNotFound =>
                write!(f, "Todo list not found"),
            EvelynDatabaseError::LookupTodoList(ref e) =>
                write!(f, "Failed to lookup todo list: {}", e),
        }
    }
}

impl error::Error for EvelynDatabaseError {
    fn description(&self) -> &str {
        match *self {
            EvelynDatabaseError::SerialisationFailed =>
                "Failed to serialise data for storage.",

            // User
            EvelynDatabaseError::InsertUser(_) =>
                "Failed to create record for new user",
            EvelynDatabaseError::LookupUser(_) =>
                "Failed to lookup user",

            // Simple Task
            EvelynDatabaseError::InsertSimpleTask(_) =>
                "Failed to create new simple task",
            EvelynDatabaseError::UpdateSimpleTask(_) =>
                "Failed to update simple task",
            EvelynDatabaseError::LookupSimpleTask(_) =>
                "Failed to lookup simple task",

            // Todo list
            EvelynDatabaseError::InsertTodoList(_) =>
                "Failed to insert todo list",
            EvelynDatabaseError::AddItemToTodoList(_) =>
                "Failed to add item to todo list",
            EvelynDatabaseError::LookupTodoLists(_) =>
                "Failed to lookup todo lists",
            EvelynDatabaseError::TodoListNotFound =>
                "Todo list not found",
            EvelynDatabaseError::LookupTodoList(_) =>
                "Failed to lookup todo list",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            EvelynDatabaseError::SerialisationFailed => None,

            // User
            EvelynDatabaseError::InsertUser(ref err) => Some(err),
            EvelynDatabaseError::LookupUser(ref err) => Some(err),

            // Simple Task
            EvelynDatabaseError::InsertSimpleTask(ref err) => Some(err),
            EvelynDatabaseError::UpdateSimpleTask(ref err) => Some(err),
            EvelynDatabaseError::LookupSimpleTask(ref err) => Some(err),

            // Todo List
            EvelynDatabaseError::InsertTodoList(ref err) => Some(err),
            EvelynDatabaseError::AddItemToTodoList(ref err) => Some(err),
            EvelynDatabaseError::LookupTodoLists(ref err) => Some(err),
            EvelynDatabaseError::TodoListNotFound => None,
            EvelynDatabaseError::LookupTodoList(ref err) => Some(err),
        }
    }
}
