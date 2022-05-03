use bridge::Bridge;

pub mod models {
    use bridge::{Model, Field, Service};
    use bridge::{Http, Wrapper, Method, WrapperMode, WrapperType, FieldType};

    pub fn model_user<'a>() -> Model<'a> {
        let mut user = Model::new();
        
        // data
        let fields = vec![
            Field::new(
                "username",
                "Username",
                FieldType::Text,
                Wrapper::new(WrapperType::Input, WrapperMode::Default("default"))
            ),
            Field::new(
                "password",
                "Password",
                FieldType::Password,
                Wrapper::new(WrapperType::Input, WrapperMode::Default("default"))
            ),
        ];
        user.add_fields(fields);

        let services = vec![
            Service::new(
                "get",
                None,
                Http::new(Method::GET, "/user/get", false, false)
            ),
            Service::new(
                "create",
                Some(vec![0, 1]),
                Http::new(Method::POST, "/user/create", true, false)
            ),
            Service::new(
                "update",
                Some(vec![0, 1]),
                Http::new(Method::PUT, "/user/update/{id}", true, false)
            ),
            Service::new(
                "delete",
                None,
                Http::new(Method::DELETE, "/user/delete/{id}", true, false)
            ),
        ];
        user.add_services(services);

        user
    }
}


fn main() {
    let api = Bridge::new(vec![
        models::model_user()
    ]);
    
    println!("api rustified!\n- registered models -> {}", api.models.unwrap().len());
}