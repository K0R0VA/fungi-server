use crate::model::client::{NewUser, Client, InputNewUser};
use crate::middleware::graphql::schema::AppContext;

use juniper::{ FieldError, graphql_value};
use validator::{Validate};

pub(crate) struct Mutation;

#[juniper::graphql_object(Context = AppContext)]
impl Mutation {
    async fn create_client(input: InputNewUser, context: &AppContext) -> Result<Client, FieldError> {
        match Validate::validate(&input) {
            Ok(_) => {
                let hash_pass = context.descriptor.hash_password(input.password).await;
                let new_user = NewUser::new((&input.username, &input.email), &hash_pass);
                context.database.pg_send(new_user).await
            }
            Err(e) => {
                Err(FieldError::new(format!("{:?}", e), graphql_value!("")))
            }
        }
    }
}