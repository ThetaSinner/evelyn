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
    ReqestForActionWhichEvelynDoesNotKnowHowToDo(EvelynBaseError),
    EvelynTriedToHandleTheRequestButDidNotYieldAResponse(EvelynBaseError),
    ExpectedHeaderOnRequestButNoneWasFound(EvelynBaseError),
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
    UpdateTodoListItem(EvelynCoreError),
}

macro_rules! EvelynServiceErrorDisplay {
    ($({$x:ident::$x2:ident, $y:expr}),+) => {
        impl fmt::Display for EvelynServiceError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    $($x::$x2(_) => write!(f, $y)),*
                }
            }
        }

        impl error::Error for EvelynServiceError {
            fn description(&self) -> &str {
                match *self {
                    $($x::$x2(_) => $y),*
                }
            }

            fn cause(&self) -> Option<&error::Error> {
                match *self {
                    $($x::$x2(ref e) => Some(e)),*
                }
            }
        }
    }
}

EvelynServiceErrorDisplay!{
    // Processing layer.
    {EvelynServiceError::ReqestForActionWhichEvelynDoesNotKnowHowToDo, "100001"},
    {EvelynServiceError::EvelynTriedToHandleTheRequestButDidNotYieldAResponse, "100002"},
    {EvelynServiceError::ExpectedHeaderOnRequestButNoneWasFound, "100003"},
    {EvelynServiceError::CouldNotDecodeTheRequestPayload, "100101"},

    // User
    {EvelynServiceError::CreateUser, "100201"},
    {EvelynServiceError::UserAlreadyExists, "100202"},
    {EvelynServiceError::LogonUser, "100203"},
    {EvelynServiceError::FailedToLogonUser, "100204"},

    // Simple Task
    {EvelynServiceError::FailedToCreateSimpleTask, "100301"},
    {EvelynServiceError::FailedToUpdateSimpleTask, "100302"},
    {EvelynServiceError::FailedToLookupSimpleTask, "100303"},

    // Todo List
    {EvelynServiceError::CreateTodoList, "100401"},
    {EvelynServiceError::AddItemToTodoList, "100402"},
    {EvelynServiceError::LookupTodoLists, "100403"},
    {EvelynServiceError::LookupTodoList, "100404"},
    {EvelynServiceError::UpdateTodoListItem, "100405"}
}

#[derive(Debug)]
pub enum EvelynCoreError {
    // user
    WillNotCreateUserBecauseUserAlreadyExists(EvelynBaseError),
    CannotCheckIfUserExistsSoWillNotCreateNewUser(EvelynDatabaseError),
    FailedToCreateUser(EvelynDatabaseError),
    InvalidLogon(EvelynBaseError),
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
    FailedToUpdateTodoListItem(EvelynDatabaseError),
}

macro_rules! EvelynCoreErrorDisplay {
    ($({$x:ident::$x2:ident, $y:expr}),+) => {
        impl fmt::Display for EvelynCoreError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    $($x::$x2(ref e) => write!(f, $y, e)),*
                }
            }
        }

        impl error::Error for EvelynCoreError {
            fn description(&self) -> &str {
                match *self {
                    $($x::$x2(_) => $y),*
                }
            }

            fn cause(&self) -> Option<&error::Error> {
                match *self {
                    $($x::$x2(ref e) => Some(e)),*
                }
            }
        }
    }
}

EvelynCoreErrorDisplay!{
    //User
    {EvelynCoreError::WillNotCreateUserBecauseUserAlreadyExists, "Will not create the requested user because that user already exists. {}"},
    {EvelynCoreError::CannotCheckIfUserExistsSoWillNotCreateNewUser, "Cannot check if the user exists so a new user will not be ceated: {}"},
    {EvelynCoreError::FailedToCreateUser, "Failed to create user: {}"},
    {EvelynCoreError::InvalidLogon, "Invalid logon {}"},
    {EvelynCoreError::FailedToLogonUser, "Failed to logon user: {}"},

    // Simple Task
    {EvelynCoreError::FailedToCreateSimpleTask, "Failed to create task: {}"},
    {EvelynCoreError::FailedToUpdateSimpleTask, "Failed to update task: {}"},
    {EvelynCoreError::FailedToLookupSimpleTask, "Failed to lookup task: {}"},

    //Todo List
    {EvelynCoreError::FailedToCreateTodoList, "Failed to create todo list: {}"},
    {EvelynCoreError::FailedToAddItemToTodoList, "Failed to add item to todo list: {}"},
    {EvelynCoreError::FailedToLookupTodoLists, "Failed to lookup todo lists: {}"},
    {EvelynCoreError::FailedToLookupTodoList, "Failed to lookup todo list: {}"},
    {EvelynCoreError::FailedToUpdateTodoListItem, "Failed to update todo list item: {}"}
}


#[derive(Debug)]
pub enum EvelynDatabaseError {
    SerialisationFailed(EvelynBaseError),

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
    TodoListNotFound(EvelynBaseError),
    LookupTodoList(MongoDbError),
    UpdateTodoListItem(MongoDbError),
}

macro_rules! EvelynDatabaseErrorDisplay {
    ($({$x:ident::$x2:ident, $y:expr}),+) => {
        impl fmt::Display for EvelynDatabaseError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    $($x::$x2(ref e) => write!(f, $y, e)),*
                }
            }
        }

        impl error::Error for EvelynDatabaseError {
            fn description(&self) -> &str {
                match *self {
                    $($x::$x2(_) => $y),*
                }
            }

            fn cause(&self) -> Option<&error::Error> {
                match *self {
                    $($x::$x2(ref e) => Some(e)),*
                }
            }
        }
    }
}

EvelynDatabaseErrorDisplay!{
    {EvelynDatabaseError::SerialisationFailed, "Failed to serialise data for storage. {}"},

    // User
    {EvelynDatabaseError::InsertUser, "Failed to create record for new user\n {}"},
    {EvelynDatabaseError::LookupUser, "Failed to lookup user: {}"},

    // Simple Task
    {EvelynDatabaseError::InsertSimpleTask, "Failed to create new simple task: {}"},
    {EvelynDatabaseError::UpdateSimpleTask, "Failed to update simple task: {}"},
    {EvelynDatabaseError::LookupSimpleTask, "Failed to lookup simple tasks: {}"},

    // Todo List
    {EvelynDatabaseError::InsertTodoList, "Failed to insert todo list: {}"},
    {EvelynDatabaseError::AddItemToTodoList, "Failed to add item to todo list: {}"},
    {EvelynDatabaseError::LookupTodoLists, "Failed to lookup todo lists: {}"},
    {EvelynDatabaseError::TodoListNotFound, "Todo list not found {}"},
    {EvelynDatabaseError::LookupTodoList, "Failed to lookup todo list:  {}"},
    {EvelynDatabaseError::UpdateTodoListItem, "Failed to update todo list item:  {}"}
}


// This error is a null enum that gets passed if there is no specific error to pass up the chain
// Side effect of using macros to generate code
#[derive(Debug)]
pub enum EvelynBaseError {
    NothingElse
}

macro_rules! EvelynBaseErrorDisplay {
    ($({$x:ident::$x2:ident, $y:expr}),+) => {
        impl fmt::Display for EvelynBaseError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    $($x::$x2 => write!(f, $y)),*
                }
            }
        }

        impl error::Error for EvelynBaseError {
            fn description(&self) -> &str {
                match *self {
                    $($x::$x2 => $y),*
                }
            }

            fn cause(&self) -> Option<&error::Error> {
                match *self {
                    $($x::$x2 => None),*
                }
            }
        }
    }
}

EvelynBaseErrorDisplay!({EvelynBaseError::NothingElse, ""});
