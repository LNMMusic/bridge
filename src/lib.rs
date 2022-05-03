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
impl Wrapper {
    // constr
    pub fn new(type_: WrapperType, mode: WrapperMode) -> Self {
        Self { type_, mode }
    }
}
pub enum FieldType {
    Text,
    Email,
    Number,
    Password,
}
pub struct Field<'a> {
    id:             usize,
    pub name:       &'a str,
    pub label:      &'a str,
    pub type_:      FieldType,
    pub wrapper:    Wrapper,
}
impl<'a> Field<'a> {
    // constr
    pub fn new(name: &'a str, label: &'a str, type_: FieldType, wrapper: Wrapper) -> Self {
        Self { id: 0, name, label, type_, wrapper }
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
    pub title:      &'a str,
    pub fields:     Option<Vec<usize>>,
    pub http:       Http,
}
impl<'a> Service<'a> {
    // constr
    pub fn new(title: &'a str, fields: Option<Vec<usize>>, http: Http) -> Self {
        Self { title, fields, http }
    }
}



// MODEL
type Fields<'a> = Option<Vec<Field<'a>>>;
type Services<'a> = Option<Vec<Service<'a>>>;

pub struct Model<'a> {
    pub fields:     Fields<'a>,
    pub services:   Services<'a>,
}
impl<'a> Model<'a> {
    // constr
    pub fn new() -> Self {
        Self { fields: None, services: None }
    }
    // methods
    pub fn add_field(&mut self, mut field: Field<'a>) {
        if let Some(ref mut fields) = self.fields {
            field.id = fields.len();
            fields.push(field)
        } else {
            self.fields = Some(vec![field])
        }
    }
    pub fn add_fields(&mut self, fields: Vec<Field<'a>>) {
        for field in fields {
            self.add_field(field)
        }
    }
    pub fn add_service(&mut self, service: Service<'a>) {
        if let Some(ref mut services) = self.services {
            services.push(service)
        } else {
            self.services = Some(vec![service])
        }
    }
    pub fn add_services(&mut self, services: Vec<Service<'a>>) {
        for service in services {
            self.add_service(service)
        }
    }
}


type Models<'a> = Option<Vec<Model<'a>>>;
pub struct Bridge<'a> {
    pub models:     Models<'a>,
}
impl<'a> Bridge<'a> {
    // constr
    pub fn new(models: Vec<Model<'a>>) -> Self {
        Self { models: Some(models) }
    }
    // methods
    pub fn add_model(&mut self, model: Model<'a>) {
        if let Some(ref mut models) = self.models {
            models.push(model)
        } else {
            self.models = Some(vec![model])
        }
    }
    pub fn add_models(&mut self, models: Vec<Model<'a>>) {
        for model in models {
            self.add_model(model)
        }
    }
}