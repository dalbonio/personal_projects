use juniper::{EmptyMutation, RootNode, EmptySubscription, GraphQLObject};
use diesel::{Queryable};
use juniper::FieldResult;

extern crate dotenv;

use std::env;

use diesel::pg::PgConnection;
use diesel::Connection;
use diesel::prelude::*;
use dotenv::dotenv;
    
    
#[derive(Queryable)]
#[derive(GraphQLObject)]
#[graphql(description = "A simple address")]
struct Address {
    id: i32,
    street: String,
    number: String,
    city: String,
    client_id: i32,
}

#[derive(Queryable)]
struct Client {
    id: i32,
    name: String,
}

#[juniper::graphql_object]
impl Client {

    fn id(&self) -> &i32 {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn addresses(&self) -> Vec<Address> {
        use crate::schema::addresses::dsl::*;
        let connection = establish_connection();
        addresses
          .filter(client_id.eq(self.id))
          .limit(100)
          .load::<Address>(&connection)
          .expect("Error loading addresses")
      }
}


fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {

    fn version() -> FieldResult<String> {
        Ok( String::from("0.0.1"))
    }

    fn clients() -> FieldResult<Vec<Client>> {
        let connection = establish_connection();
        use crate::schema::clients::dsl::*;
        Ok(
        clients
            .limit(100)
            .load::<Client>(&connection)
            .expect("Error loading members")
        )
    }
    
    fn client(user_id: i32) -> FieldResult<Client> {
        let connection = establish_connection();
        use crate::schema::clients::dsl::*;
        Ok(clients
            .find(user_id)
            .first(&connection)
            .expect("Error loading members")
        )
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn one_result() {
    }
}


