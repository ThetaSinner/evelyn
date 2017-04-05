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

extern crate evelyn;

use evelyn::core::token_service::TokenService;

#[test]
pub fn create_and_decode_session_token_using_the_token_service() {
    let token_service = TokenService::new(String::from("my secret token"));

    let user = evelyn::model::UserModel{
        user_name: String::from("the username"),
        email_address: String::from("the email address"),
        password: String::from("the password"),
    };

    let session_token = token_service.create_session_token(&user);

    let decoded = token_service.extract_session_token(&session_token);

    assert_eq!(decoded.user_id, "the email address");
}
