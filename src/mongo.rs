use std::env;
use serde_derive::{Deserialize, Serialize};
use mongodb::{Client, Collection, options::ClientOptions, bson::doc, error::Error, results::{UpdateResult, InsertOneResult}};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub guild_id: String,
    pub user_id: String,
    pub roles: Vec<String>,
}

pub struct Database {
    pub client: Client
}

impl Database {
    pub async fn get_user(&self, guild_id: &String, user_id: &String) -> Result<Option<User>, Error> {
        let users: Collection<User> = self.client.database("wendigo").collection("users");
        let res = users.find_one(doc! { "guildId": guild_id, "userId": user_id }, None).await?;
        match res {
            Some(user) => {
                Ok(
                    Some(
                        User {
                            guild_id: user.guild_id,
                            user_id: user.user_id,
                            roles: user.roles,
                        }
                    )
                )
            },
            None => Ok(None)
        }
    }

    pub async fn update_user(&self, guild_id: &String, user_id: &String, roles: &Vec<String>) -> Result<UpdateResult, Error> {
        let users: Collection<User> = self.client.database("wendigo").collection("users");
        return users.update_one(doc! { "guildId": guild_id, "userId": user_id }, doc! { "$set": { "roles": roles } }, None).await
    }

    pub async fn create_user(&self, guild_id: &String, user_id: &String, roles: &Vec<String>) -> Result<InsertOneResult, Error> {
        let users: Collection<User> = self.client.database("wendigo").collection("users");
        return users.insert_one(User { guild_id: guild_id.to_string(), user_id: user_id.to_string(), roles: roles.to_vec() }, None).await;
    }
}

pub async fn connect() -> Database {
    let uri = env::var("MONGO_URI").expect("Expected a MongoDB URI in the environment");
    let mut client_options = ClientOptions::parse(&uri).await.expect("Failed to parse MongoDB URI");
    client_options.app_name = Some("Wendigo".to_string());
    let client = Client::with_options(client_options).expect("Failed to initialize MongoDB client");
    Database { client }
}