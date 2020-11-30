use crate::middleware::graphql::schema::AppContext;
use crate::model::client::{SignIn, Client};
use juniper::{FieldError, graphql_value };
use validator::Validate;

pub(crate) struct Query;

const ERROR_MESSAGE: &str = "password or email not verified";

fn if_verified(condition: bool, client: Client) -> Result<Client, FieldError> {
    if condition {
        Ok(client)
    }
    else {
        Err(FieldError::new(ERROR_MESSAGE, graphql_value!("")))
    }
}

#[juniper::graphql_object(Context = AppContext)]
impl Query {
    pub async fn api_version() -> &str {
        "1.0"
    }
    pub async fn login_client(client: SignIn, context: &AppContext) -> Result<Client, FieldError> {
        match Validate::validate(&client) {
            Ok(_) => {
                let password = client.password.clone();
                match context.database.pg_send(client).await  {
                    Ok(client) => if_verified(context.descriptor.verify_password(password, &client.password.clone()).await, client),
                    Err(e) => Err(FieldError::new(ERROR_MESSAGE, graphql_value!("")))
                }
            }
            Err(e) => {
                Err(FieldError::new(format!("{:?}", e), graphql_value!("")))
            }
        }
    }
}

