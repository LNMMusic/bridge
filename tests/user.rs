use bridge::{Model, Field, Wrapper};
use bridge::{Http};
use bridge::{FieldType, WrapperType, WrapperMode, Method};

#[test]
fn user_services() {
    // Model Fields
    let user_fields = vec![
        Field {
            name:       "username",
            label:      "Username",
            type_:      FieldType::Text,
            wrapper:    Wrapper {
                type_:  WrapperType::Input,
                mode:   WrapperMode::Default("test"),
            },
            validator:  false,
        },
        Field {
            name:       "password",
            label:      "Password",
            type_:      FieldType::Password,
            wrapper:    Wrapper {
                type_:  WrapperType::Input,
                mode:   WrapperMode::Default("test"),
            },
            validator:  false,
        },
    ];

    // Model Services
    let mut user_model = Model::new();
    // add services
    user_model.add_service(
        "get",
        None,
        None,
        Http::new(Method::GET, "/user/get", false, false)
    );
    user_model.add_service(
        "create",
        Some(vec!["username".to_owned(), "password".to_owned()]),
        Some(&user_fields),
        Http::new(Method::GET, "/user/get", false, false)
    );

    // get info services
    let services = user_model.service.unwrap();
    for s in &services {
        s.display_fields()
    }
}