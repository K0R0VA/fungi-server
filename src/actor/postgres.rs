use crate::model::postgre::schema::{creator, project, plugin, comment, liked, import};
use crate::model::postgre::project::{Project, NewProject, ClientIdForProjects, SaveProject};
use crate::model::postgre::client::{SignIn, NewUser, Creator};
use crate::model::postgre::plugin::{NewPlugin, Plugin, DeletePlugin, EditPlugin, NewComment, Comment, Like, Liked, DisLike, GetComments, GetPlugins, GetPluginInfo, AddDependency, Import, DeleteDependency};

use actix::{Actor, SyncContext, Handler };
use diesel::{PgConnection, QueryResult, RunQueryDsl, QueryDsl, ExpressionMethods};
use diesel::r2d2::{ConnectionManager, Pool};
use chrono::Utc;
use uuid::Uuid;

pub struct PgActor {
    pool :Pool<ConnectionManager<PgConnection>>,
}

impl PgActor {
    pub fn new(pool : Pool<ConnectionManager<PgConnection>>) -> Self {
        PgActor {
            pool,
        }
    }
}



impl Actor for PgActor {
    type Context = SyncContext<Self>;
}

impl Handler<NewUser> for PgActor {
    type Result = QueryResult<Creator>;

    fn handle(&mut self, msg: NewUser, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        let client : Creator = Creator::new(&msg.username, &msg.email, &msg.password);
        diesel::insert_into(creator::table)
            .values::<Creator>(client)
            .get_result(conn)
    }
}

impl Handler<SignIn> for PgActor {
    type Result = QueryResult<Creator>;

    fn handle(&mut self, msg: SignIn, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        creator::table
            .filter(creator::email.eq(msg.email))
            .first::<Creator>(conn)
    }
}

impl Handler<ClientIdForProjects> for PgActor {

    type Result = QueryResult<Vec<Project>>;

    fn handle(&mut self, msg: ClientIdForProjects, _ : &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        project::table
            .filter(project::creator_id.eq(msg.id))
            .load::<Project>(conn)
    }
}

impl Handler<NewProject> for PgActor {
    type Result = QueryResult<Project>;

    fn handle(&mut self, msg: NewProject, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        let project = Project::new(msg.name_project, msg.id_client);
        diesel::insert_into(project::table)
            .values::<Project>(project)
            .get_result(conn)
    }
}

impl Handler<SaveProject> for PgActor {
    type Result = QueryResult<usize>;

    fn handle(&mut self, msg: SaveProject, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        let time = Utc::now().date().naive_local();
        diesel::update(project::table)
            .set(project::last_update.eq(time))
            .filter(project::id.eq(msg.0))
            .execute(conn)
    }
}

impl Handler<NewPlugin> for PgActor {
    type Result = QueryResult<Plugin>;

    fn handle(&mut self, msg: NewPlugin, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        let plugin = Plugin::new(msg.name, msg.creator, msg.public);
        diesel::insert_into(plugin::table)
            .values::<Plugin>(plugin)
            .get_result(conn)
    }
}

impl Handler<DeletePlugin> for PgActor {
    type Result = QueryResult<usize>;

    fn handle(&mut self, msg: DeletePlugin, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        diesel::delete(plugin::table)
            .filter(plugin::id.eq(msg.id_plugin))
            .execute(conn)
    }
}

impl Handler<EditPlugin> for PgActor {
    type Result = QueryResult<Plugin>;

    fn handle(&mut self, msg: EditPlugin, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        diesel::update(plugin::table)
            .filter(plugin::id.eq(msg.id_plugin))
            .set((plugin::name.eq(msg.name), plugin::definition.eq(msg.definition), plugin::public.eq(msg.public)))
            .get_result(conn)
    }
}

impl Handler<GetPlugins> for PgActor {
    type Result = QueryResult<Vec<Plugin>>;

    fn handle(&mut self, _: GetPlugins, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        plugin::table
            .load::<Plugin>(conn)
    }
}

impl Handler<GetPluginInfo> for PgActor {
    type Result = QueryResult<Plugin>;

    fn handle(&mut self, msg: GetPluginInfo, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        plugin::table
            .filter(plugin::id.eq(msg.plugin_id))
            .first::<Plugin>(conn)
    }
}

impl Handler<NewComment> for PgActor {
    type Result = QueryResult<Comment>;

    fn handle(&mut self, msg: NewComment, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        let comment = Comment {id: msg.id, user_id: msg.user_id,plugin_id: msg.plugin_id, content: msg.content, like_count: 0};
        diesel::insert_into(comment::table)
            .values(comment)
            .get_result(conn)
    }
}

impl Handler<Like> for PgActor {
    type Result = QueryResult<usize>;

    fn handle(&mut self, msg: Like, _: &mut Self::Context) -> Self::Result {
        use crate::model::postgre::schema::comment::dsl::*;
        let conn : &PgConnection = &self.pool.get().unwrap();
        let liked = Liked {id: Uuid::new_v4(), user_id: msg.user_id, comment_id: msg.comment_id};
        diesel::insert_into(liked::table).values(liked).execute(conn)?;
        diesel::update(comment)
            .filter(id.eq(msg.comment_id))
            .set(like_count.eq(like_count + 1))
            .execute(conn)
    }
}

impl Handler<DisLike> for PgActor {
    type Result = QueryResult<usize>;

    fn handle(&mut self, msg: DisLike, _: &mut Self::Context) -> Self::Result {
        use crate::model::postgre::schema::comment::dsl::*;
        let conn : &PgConnection = &self.pool.get().unwrap();
        diesel::delete(liked::table)
            .filter(liked::comment_id.eq(msg.comment_id))
            .filter(liked::user_id.eq(msg.user_id))
            .execute(conn)?;
        diesel::update(comment)
            .filter(id.eq(msg.comment_id))
            .set(like_count.eq(like_count - 1))
            .execute(conn)
    }
}

impl Handler<GetComments> for PgActor {
    type Result = QueryResult<Vec<Comment>>;

    fn handle(&mut self, msg: GetComments, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        comment::table
            .filter(comment::plugin_id.eq(msg.id_plugin))
            .load::<Comment>(conn)
    }
}

impl Handler<AddDependency> for PgActor {
    type Result = QueryResult<usize>;

    fn handle(&mut self, msg: AddDependency, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        let import = Import {id: Uuid::new_v4(), plugin_id: msg.id_plugin, project_id: msg.id_project};
        diesel::insert_into(import::table)
            .values(import)
            .execute(conn)
    }
}

impl Handler<DeleteDependency> for PgActor {
    type Result = QueryResult<usize>;

    fn handle(&mut self, msg: DeleteDependency, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        diesel::delete(import::table)
            .filter(import::plugin_id.eq(msg.id_plugin))
            .filter(import::project_id.eq(msg.id_project))
            .execute(conn)
    }
}







