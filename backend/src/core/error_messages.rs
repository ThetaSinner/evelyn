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

macro_rules! EvelynErrorDisplay {
    // Both error codes and error messages
    ($x:ident, $({$x2:ident, $y:expr, $z:expr}),+) => (
        impl fmt::Display for $x {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self { $($x::$x2(_) => write!(f, $y)),* }
            }
        }

        impl error::Error for $x {
            fn description(&self) -> &str {
                match *self { $($x::$x2(_) => $z),* }
            }

            fn cause(&self) -> Option<&error::Error> {
                match *self { $($x::$x2(ref e) => Some(e)),* }
            }
        }
    );

    // Only error message, Error that refers to another error
    ($x:ident, $({$x2:ident, $y:expr}),+) => (
        impl fmt::Display for $x {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self { $($x::$x2(ref e) => write!(f, $y, e)),* }
            }
        }

        impl error::Error for $x {
            fn description(&self) -> &str {
                match *self { $($x::$x2(_) => $y),* }
            }

            fn cause(&self) -> Option<&error::Error> {
                match *self { $($x::$x2(ref e) => Some(e)),* }
            }
        }
    );

    // Only error message, no references to other errors
    ($x:ident, $({$x2:ident}),+) => (
        impl fmt::Display for $x {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self { $($x::$x2 => write!(f, "")),* }
            }
        }

        impl error::Error for $x {
            fn description(&self) -> &str {
                match *self { $($x::$x2 => ""),* }
            }

            fn cause(&self) -> Option<&error::Error> {
                match *self { $($x::$x2 => None),* }
            }
        }
    );
}

#[derive(Debug)]
pub enum EvelynServiceError {
    ReqestForActionWhichEvelynDoesNotKnowHowToDo(EvelynBaseError),
    EvelynTriedToHandleTheRequestButDidNotYieldAResponse(EvelynBaseError),
    ExpectedHeaderOnRequestButNoneWasFound(EvelynBaseError),

    CouldNotDecodeTheRequestPayload(serde_json::Error),
    ForeignSessionToken(EvelynBaseError),

    // Server Admin
    FailedToPurge(EvelynCoreError),
    InvalidPurgeTargetType(EvelynBaseError),

    // User
    CreateUser(EvelynCoreError),
    UserAlreadyExists(EvelynCoreError),
    LogonUser(EvelynCoreError),
    FailedToLogonUser(EvelynCoreError),

    // User group
    CreateUserGroup(EvelynCoreError),

    // Simple Task
    FailedToCreateSimpleTask(EvelynCoreError),
    FailedToUpdateSimpleTask(EvelynCoreError),
    FailedToLookupSimpleTask(EvelynCoreError),

    // Todo List
    CreateTodoList(EvelynCoreError),
    AddItemToTodoList(EvelynCoreError),
    LookupTodoLists(EvelynCoreError),
    LookupTodoList(EvelynCoreError),
    UpdateTodoListItem(EvelynCoreError),

    // Calendar
    AddCalendarEvent(EvelynCoreError),
}

EvelynErrorDisplay!{
    EvelynServiceError,
    // Processing layer.
    {ReqestForActionWhichEvelynDoesNotKnowHowToDo, "100001", "Request for an action which Evelyn does now know how to do"},
    {EvelynTriedToHandleTheRequestButDidNotYieldAResponse, "100002", "Evelyn tried to handle the request but hasn't managed to give anything back"},
    {ExpectedHeaderOnRequestButNoneWasFound, "100003", "Expected a header with the request but didn't find a header"},

    {CouldNotDecodeTheRequestPayload, "100101", "Could not decode the JSON request payload"},
    {ForeignSessionToken, "100102", "The server has been restarted please log on again"},

    // Server admin.
    {FailedToPurge, "100103", "Failed to purge"},
    {InvalidPurgeTargetType, "100104", "Invalid purge target type"},

    // User
    {CreateUser, "100201", "Failed to create user"},
    {UserAlreadyExists, "100202", "Failed to create user a user with that name already exists"},
    {LogonUser, "100203", "Invalid logon"},
    {FailedToLogonUser, "100204", "Failed to logon user"},

    // User group
    {CreateUserGroup, "100601", "Failed to create user group"},

    // Simple Task
    {FailedToCreateSimpleTask, "100301", "Failed to create simple task"},
    {FailedToUpdateSimpleTask, "100302", "Failed to update simple task"},
    {FailedToLookupSimpleTask, "100303", "Failed to lookup simple task"},

    // Todo List
    {CreateTodoList, "100401", "Failed to create todo list"},
    {AddItemToTodoList, "100402", "Failed to add item to todo list"},
    {LookupTodoLists, "100403", "Failed to lookup todo lists"},
    {LookupTodoList, "100404", "Failed to lookup todo list"},
    {UpdateTodoListItem, "100405", "Failed to update todo list item"},

    // Calendar
    {AddCalendarEvent, "100501", "Failed to add calendar event"}
}

#[derive(Debug)]
pub enum EvelynCoreError {
    // Server Admin
    FailedToPurgeDatabase(EvelynDatabaseError),
    FailedToPurgeDatabaseArea(EvelynDatabaseError),

    // User
    WillNotCreateUserBecauseUserAlreadyExists(EvelynBaseError),
    CannotCheckIfUserExistsSoWillNotCreateNewUser(EvelynDatabaseError),
    FailedToCreateUser(EvelynDatabaseError),
    InvalidLogon(EvelynBaseError),
    FailedToLogonUser(EvelynDatabaseError),

    // User group
    FailedToCreateUserGroup(EvelynDatabaseError),

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

    // Calendar
    FailedToAddCalendarEvent(EvelynDatabaseError),
}

EvelynErrorDisplay!{
    EvelynCoreError,

    // Server Admin
    {FailedToPurgeDatabase, "Failed to purge database {}"},
    {FailedToPurgeDatabaseArea, "Failed to purge database area {}"},

    // User
    {WillNotCreateUserBecauseUserAlreadyExists, "Will not create the requested user because that user already exists. {}"},
    {CannotCheckIfUserExistsSoWillNotCreateNewUser, "Cannot check if the user exists so a new user will not be ceated: {}"},
    {FailedToCreateUser, "Failed to create user: {}"},
    {InvalidLogon, "Invalid logon {}"},
    {FailedToLogonUser, "Failed to logon user: {}"},

    // User Group
    {FailedToCreateUserGroup, "Failed to create user group: {}"},

    // Simple Task
    {FailedToCreateSimpleTask, "Failed to create task: {}"},
    {FailedToUpdateSimpleTask, "Failed to update task: {}"},
    {FailedToLookupSimpleTask, "Failed to lookup task: {}"},

    // Todo List
    {FailedToCreateTodoList, "Failed to create todo list: {}"},
    {FailedToAddItemToTodoList, "Failed to add item to todo list: {}"},
    {FailedToLookupTodoLists, "Failed to lookup todo lists: {}"},
    {FailedToLookupTodoList, "Failed to lookup todo list: {}"},
    {FailedToUpdateTodoListItem, "Failed to update todo list item: {}"},

    // Calendar
    {FailedToAddCalendarEvent, "Failed to add calendar event: {}"}
}

#[derive(Debug)]
pub enum EvelynDatabaseError {
    SerialisationFailed(EvelynBaseError),

    // Server Admin
    PurgeDatabase(MongoDbError),
    PurgeCollection(MongoDbError),

    // User
    InsertUser(MongoDbError),
    LookupUser(MongoDbError),

    // User group
    InsertUserGroup(MongoDbError),

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

    // Calendar
    InsertCalendarEvent(MongoDbError),
}

EvelynErrorDisplay!{
    EvelynDatabaseError,
    // Processing
    {SerialisationFailed, "Failed to serialise data for storage. {}"},

    // Server Admin
    {PurgeDatabase, "Failed to purge database {}"},
    {PurgeCollection, "Failed to purge collection {}"},

    // User
    {InsertUser, "Failed to create record for new user\n {}"},
    {LookupUser, "Failed to lookup user: {}"},

    // User group
    {InsertUserGroup, "Failed to create user group: {}"},

    // Simple Task
    {InsertSimpleTask, "Failed to create new simple task: {}"},
    {UpdateSimpleTask, "Failed to update simple task: {}"},
    {LookupSimpleTask, "Failed to lookup simple tasks: {}"},

    // Todo List
    {InsertTodoList, "Failed to insert todo list: {}"},
    {AddItemToTodoList, "Failed to add item to todo list: {}"},
    {LookupTodoLists, "Failed to lookup todo lists: {}"},
    {TodoListNotFound, "Todo list not found {}"},
    {LookupTodoList, "Failed to lookup todo list:  {}"},
    {UpdateTodoListItem, "Failed to update todo list item:  {}"},

    // Calendar
    {InsertCalendarEvent, "Failed to insert calendar event: {}"}
}


// This error is a null enum that gets passed if there is no specific error to pass up the chain
// Side effect of using macros to generate code
#[derive(Debug)]
pub enum EvelynBaseError {
    NothingElse
}

EvelynErrorDisplay!(EvelynBaseError, {NothingElse});
