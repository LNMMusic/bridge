// use std::rc::Rc;

// use bridge::{Model, Field, Wrapper};
// use bridge::{Http};
// use bridge::{FieldType, WrapperType, WrapperMode, Method};

// #[test]
// fn user_services() {

//     let mut user_model = Model::new(vec![
//         Rc::new(Field {
//             name:       "username",
//             label:      "Username",
//             type_:      FieldType::Text,
//             wrapper:    Wrapper {
//                 type_:  WrapperType::Input,
//                 mode:   WrapperMode::Default("test"),
//             },
//             validator:  false,
//         }),
//         Rc::new(Field {
//             name:       "password",
//             label:      "Password",
//             type_:      FieldType::Password,
//             wrapper:    Wrapper {
//                 type_:  WrapperType::Input,
//                 mode:   WrapperMode::Default("test"),
//             },
//             validator:  false,
//         }),
//     ]);

//     // create services
//     let service_1 = user_model.create_service(
//         "get",
//         None,
//         Http::new(Method::GET, "/user/get", false, false)
//     );
//     let service_2 = user_model.create_service(
//         "get",
//         None,
//         Http::new(Method::GET, "/user/get", false, false)
//     );

//     // add services
//     user_model.add_service(service_1);
//     user_model.add_service(service_2);

//     // add services
//     // user_model.add_service(
//     //     "get",
//     //     None,
//     //     Http::new(Method::GET, "/user/get", false, false)
//     // );
//     // user_model.add_service(
//     //     "get",
//     //     None,
//     //     Http::new(Method::GET, "/user/get", false, false)
//     // );
// }

// #[test]
// fn test_box() {
//     #[derive(Debug)]
//     struct Car {
//         name:   String,
//     };
//     let car = Car {
//         name:   "bmw".to_owned(),
//     };
//     let rc_ref = Rc::new(car);
    
//     // get pointer
//     let x = rc_ref.as_ref();
// }