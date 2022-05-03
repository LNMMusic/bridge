// FIELDS
pub enum WrapperType {
    Input,
    Select,
}
pub enum WrapperMode {
    Default(&'static str),
    API(&'static str),
}
pub struct Wrapper {
    pub type_:  WrapperType,
    pub mode:   WrapperMode,
}
pub enum FieldType {
    Text,
    Email,
    Number,
    Password,
}
pub struct Field {
    pub name:       &'static str,
    pub label:      &'static str,
    pub type_:      FieldType,
    pub wrapper:    Wrapper,
    pub validator:  bool,
}
impl Field {
    // method
    pub fn check_name(&self, name: &str) -> bool {
        if self.name != name {
            return false
        }
        true
    }
}


// SERVICE
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}
pub struct Http {
    pub method:     Method,
    pub url:        &'static str,
    pub body:       bool,
    pub auth:       bool,
}
impl Http {
    // constr
    pub fn new(method: Method, url: &'static str, body: bool, auth: bool) -> Self {
        Self { method, url, body, auth }
    }
}
pub struct Service<'a> {
    pub title:      &'static str,
    pub fields:     Option<Vec<&'a Field>>,
    pub http:       Http,
}
impl<'a> Service<'a> {
    // constr
    pub fn new(title: &'static str, fields: Option<Vec<&'a Field>>, http: Http) -> Self {
        Self { title, fields, http }
    }
    // method
    pub fn display_fields(&self) {
        print!("Fields:\n");
        if let Some(ref fields) = self.fields {
            print!("Fields:\n");
            for field in fields {
                println!("1. name: {}", field.name)
            }
        }
    }
}




pub struct Model<'a> {
    pub service:    Option<Vec<Service<'a>>>,
}
impl<'a> Model<'a> {
    // constr
    pub fn new() -> Self {
        Self { service: None }
    }
    // methods
    pub fn add_service(&mut self, title: &'static str, new_fields: Option<Vec<String>>, fields: Option<&'a Vec<Field>>, http: Http) {
        // create service
        let mut new_service = Service::new(title, None, http);

        // update service
        if let Some(new_fields) = new_fields {
            let mut vec_ref = Vec::new();

            for new_field in new_fields {
                for field in fields.unwrap() {
                    if field.check_name(new_field.as_ref()) {
                        vec_ref.push(field);
                        break;
                    }
                }
            }
            if vec_ref.len() > 0 {
                new_service.fields = Some(vec_ref);
            }
        }

        // add service
        if let Some(ref mut service) = self.service {
            service.push(new_service)
        } else {
            self.service = Some(vec![new_service])
        }
        
    }
}