extern crate evelyn;

#[macro_use]
extern crate serde_json;

extern crate hyper;

#[test]
pub fn create_user() {
    let model = evelyn::model::CreateUserModel{
        user_name: "The User Name".to_owned(),
        email_address: "The Email Address".to_owned(),
        password: "The Password".to_owned()
    };

    let req = serde_json::to_string(&model).unwrap();

    let client = hyper::Client::new();
    let res = client.post("http://localhost:8080/user/create").body(&req).send().unwrap();

    assert_eq!(res.status, hyper::Ok);

    println!("{}", req);
}
