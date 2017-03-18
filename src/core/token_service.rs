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

use jwt::{encode, decode, Header, Algorithm};
use jwt::errors::{Error};

use model::{SessionTokenModel, UserModel};

pub struct TokenService {
    private_key: String,
}

impl TokenService {
    pub fn new(private_key: String) -> Self {
        TokenService{private_key: private_key}
    }

    pub fn create_session_token(&self, user_model: &UserModel) -> String {
        let session_token_model = SessionTokenModel {
            user_id: user_model.email_address.to_owned(),
        };

        match encode(Header::default(), &session_token_model, self.private_key.as_ref()) {
            Ok(t) => t,
            Err(_) => panic!() // in practice you would return the error
        }
    }

    pub fn extract_session_token(&self, token: String) -> SessionTokenModel {
        let token_data = match decode::<SessionTokenModel>(&token, self.private_key.as_ref(), Algorithm::HS256) {
            Ok(c) => c,
            Err(err) => match err {
                Error::InvalidToken => panic!(), // Example on how to handle a specific error
                _ => panic!()
            }
        };

        println!("{:?}", token_data);
        token_data.claims
    }
}