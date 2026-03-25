use crate::domain::person::PersonId;

pub struct AddPersonCommand {
    pub name: String,
}

pub struct UpdatePersonCommand {
    pub id: PersonId,
    pub name: String,
}

pub struct RemovePersonCommand {
    pub id: PersonId,
}
