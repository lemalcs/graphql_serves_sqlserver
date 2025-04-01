use crate::db_connection::{get_friends, get_person, get_persons, update_person_name};
use async_graphql::Object;

// The `Clone` trait returns a copy of the value
// it is useful when we need to return a Person from
// a data source like a database (SQL Server in this case)
#[derive(Clone)]
pub struct Person {
    pub id: i32,
    pub name: String,
}

#[Object]
impl Person {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn friends(&self) -> Vec<Person> {
        let persons = get_friends(self.id).await.unwrap();
        persons
    }
}

pub struct Query;

#[Object]
impl Query {
    async fn persons(&self) -> async_graphql::Result<Vec<Person>> {
        match get_persons().await {
            Ok(persons) => Ok(persons),
            Err(message) => Err(async_graphql::Error::new(message.to_string())),
        }
    }

    async fn person(&self, id: i32) -> async_graphql::Result<Person> {
        match get_person(id).await {
            Ok(person) => Ok(person),
            Err(message) => Err(async_graphql::Error::new(message.to_string())),
        }
    }
}

pub struct Mutation;

#[Object]
impl Mutation{
    pub async fn update_person_name(&self, id:i32, name: String) -> async_graphql::Result<Person> {
        match update_person_name(id, name).await{
            Ok(person) => Ok(person),
            Err(message) => Err(async_graphql::Error::new(message.to_string())),
        }
    }
}
