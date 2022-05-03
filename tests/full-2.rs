use std::cell::Cell;

struct Field {
    name:       String,
}

struct Model<'a> {
    fields:     Vec<Field>,             // base fields to be referenced by services
    selected:   Cell<Option<&'a Field>>,
}
impl<'a> Model<'a> {
    // constr
    fn new(fields: Vec<Field>) -> Self {
        Self { fields, selected: Cell::new(None) }
    }
    // methods
    fn select(&'a self, name: String) {
        for field in &self.fields {
            if field.name == name {
                self.selected.set(Some(field))
            }
        }
    }
}

pub fn created() -> Model {
    let mut user = Model::new(vec![
        Field { name: "username".to_owned() },
        Field { name: "password".to_owned() },
    ]);

    user.select("username".to_owned());
    println!("selected -> {}", user.selected.get().unwrap().name);
    
    user.select("password".to_owned());
    println!("selected -> {}", user.selected.get().unwrap().name);

    user
}

#[test]
fn testing() {
    let user = created()
    
}

