use std::{rc::Rc, sync::PoisonError, vec};

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
}




pub struct Model<'a> {
    pub fields:     Vec<Rc<Field>>,
    pub service:    Option<Vec<Service<'a>>>,
}
impl<'a> Model<'a> {
    // constr
    pub fn new(fields: Vec<Rc<Field>>) -> Self {
        Self { fields, service: None }
    }
    // methods
    pub fn create_service(&self, title: &'static str, fields: Option<Vec<String>>, http: Http) -> Service {
        // create service
        let mut new_service = Service::new(title, None, http);

        // update service with vec pointers
        if let Some(fields) = fields {
            let mut vec_ref = Vec::new();

            for field in fields {
                for f in &self.fields {
                    if field.as_str() == f.name {
                        vec_ref.push(f.as_ref());
                        break;
                    }
                }
            }
            if vec_ref.len() > 0 {
                new_service.fields = Some(vec_ref);
            }
        };

        // service
        new_service
    }
    pub fn add_service(&mut self, new_service: Service<'a>) {
        if let Some(ref mut service) = self.service {
            service.push(new_service);
        } else {
            self.service = Some(vec![new_service]);
        }
    }
}