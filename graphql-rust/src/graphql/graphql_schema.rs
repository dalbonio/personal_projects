use juniper::{EmptyMutation, RootNode, EmptySubscription, GraphQLObject};
use juniper::FieldResult;

#[derive(GraphQLObject)]
#[graphql(description = "A simple test user with name and id")]
struct Member {
    id: i32,
    name: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn members() -> FieldResult<Vec<Member>> {
        Ok(vec![
            Member {
                id: 1,
                name: "User1".to_owned(),
            },
            Member {
                id: 2,
                name: "User2".to_owned(),
            }
        ])
    }

    fn human(_id: i32) -> FieldResult<Member> {
        Ok(Member {
            id: 1,
            name: "User1".to_owned(),
        })
    }

    fn member(_id: i32) -> FieldResult<Member> {
        Ok(Member {
            id: 2,
            name: "User2".to_owned(),
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}

