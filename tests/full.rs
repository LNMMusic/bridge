struct Field {
    name:       String,
}

struct Model<'a> {
    fields:     Vec<Field>,         // base fields to be referenced by services
    selected:   Option<&'a Field>,
}
impl<'a> Model<'a> {
    // constr
    fn new(fields: Vec<Field>) -> Self {
        Self { fields, selected: None }
    }
    // methods
    fn select_field(&'a mut self, name: String) {
        for field in &self.fields {
            if field.name == name {
                self.selected = Some(field);
                break;
            }
        }
    }
}


#[test]
fn testing() {
    let mut user = Model::new(vec![
        Field { name: "username".to_owned() },
        Field { name: "password".to_owned() },
    ]);

    user.select_field("username".to_owned());
    user.select_field("password".to_owned());
}