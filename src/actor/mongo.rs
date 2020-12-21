use super::super::model::mongo::input_project::{NewProject, MongoProject as InputMongoProject, SaveProject};
use super::super::model::mongo::get_project::{MongoProject as GetMongoProject, GetProject};

use mongodb::{Client};
use actix::{Actor, Context, Handler, ResponseFuture};
use bson::{doc, Document, from_document};
use mongodb::results::{UpdateResult, InsertOneResult};
use mongodb::error::{Error};

pub struct MongoActor (pub Client);

impl Actor for MongoActor {
    type Context = Context<Self>;
}

impl Handler<NewProject> for MongoActor {

    type Result = ResponseFuture<Result<InsertOneResult, Error>>;

    fn handle(&mut self, msg: NewProject, _: &mut Self::Context) -> Self::Result {
        let database = self.0.database("Fungi");
        let collection = database.collection("Projects");
        let project = InputMongoProject::default();
        let bson = bson::to_bson(&project).unwrap();
        let doc = doc! {"_id": format!("{}", msg.0), "project": bson};
        Box::pin(
            async move {
                collection.insert_one(doc, None ).await
            }
        )
    }
}

impl Handler<SaveProject> for MongoActor {

    type Result = ResponseFuture<Result<UpdateResult, Error>>;

    fn handle(&mut self, msg: SaveProject, _: &mut Self::Context) -> Self::Result {
        let database = self.0.database("Fungi");
        let collection = database.collection("Projects");
        let bson = bson::to_bson(&msg.inner).unwrap();
        let key =  format!("{}", msg.id);
        let filter = doc! {"_id" : key };
        let update = doc! {"project" : bson};
        Box::pin(
            async move {
                collection.update_one(filter, update, None ).await
            }
        )
    }
}

impl Handler<GetProject> for MongoActor {

    type Result = ResponseFuture<Result<GetMongoProject, Vec<String>>>;

    fn handle(&mut self, msg: GetProject, _: &mut Self::Context) -> Self::Result {
        let database = self.0.database("Fungi");
        let collection = database.collection("Projects");
        let key =  format!("{}", msg.id);
        let filter = doc! {"_id" : key };
        Box::pin(
            async move {
                match collection.find_one(filter, None ).await
                    .map(|option:Option<Document>| option
                        .map(|document| from_document(document).ok()
                        )
                    ) {
                    Ok(option) => {
                        match option {
                            Some(option) => {
                                match option {
                                    Some(project) => project,
                                    None => Err(vec!["Can't parse document".to_owned()])
                                }
                            },
                            None => Err(vec!["can't found document".to_owned()])
                        }
                    },
                    Err(e) => Err(e.labels().to_vec())
                }
            }
        )
    }
}










