use juniper::{EmptyMutation, RootNode, EmptySubscription, GraphQLObject};
use juniper::FieldResult;

#[derive(GraphQLObject)]
#[graphql(description = "A client of the websystem")]
struct Client {
    id: i32,
    address: Address,
    name: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "A simple address")]
struct Address {
    street: String,
    number: String,
    city: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {

    fn version() -> FieldResult<String> {
        Ok( String::from("0.0.1"))
    }

    fn clients() -> FieldResult<Vec<Client>> {
        Ok(vec![
            Client {
                id: 1,
                address: Address{street: String::from("Street1"), number: String::from("30"), city: String::from("City1")},
                name: "User1".to_owned(),
            },
            Client {
                id: 2,
                address: Address{street: String::from("Street2"), number: String::from("30"), city: String::from("City2")},
                name: "User2".to_owned(),
            }
        ])
    }

    fn client(_id: i32) -> FieldResult<Client> {
        Ok(Client {
            id: 2,
            address: Address{street: String::from("Street2"), number: String::from("30"), city: String::from("City2")},
            name: "User2".to_owned(),
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}

