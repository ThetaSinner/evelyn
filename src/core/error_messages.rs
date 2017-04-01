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

use std::fmt;

#[derive(Debug)]
enum EvelynServiceError {
    ReqestForActionWhichEvelynDoesNotKnowHowToDo,
    EvelynTriedToHandleTheRequestButDidNotYieldAResponse,
}

impl fmt::Display for EvelynServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EvelynServiceError::ReqestForActionWhichEvelynDoesNotKnowHowToDo(ref err) => {
                write!(f, "Request for an action which Evelyn does now know how to do")
            },
            EvelynServiceError::EvelynTriedToHandleTheRequestButDidNotYieldAResponse(ref err) => {
                write!(f, "Evelyn tried to handle the request but hasn't managed to give anything back")
            },
        }
    }
}
