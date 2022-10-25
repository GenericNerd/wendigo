use serenity::model::{prelude::*};
use serenity::prelude::Context;
use crate::Handler;

impl Handler {
    pub async fn guild_member_update(&self, old: Option<Member>, new: Member) {
        let user = self.db.get_user(&new.guild_id.to_string(), &new.user.id.to_string()).await;
        match user {
            Ok(user) => {
                if let Some(old) = old {
                    if old.roles == new.roles {
                        return;
                    }
                }
                let mut role_ids: Vec<String> = Vec::new();
                for role in new.roles {
                    role_ids.push(role.to_string());
                }
                if user.is_some() {
                    let res = self.db.update_user(&new.guild_id.to_string(), &new.user.id.to_string(), &role_ids).await;
                    match res {
                        Ok(_) => {},
                        Err(e) => println!("Error updating user: {}", e)
                    }
                } else {
                    let res = self.db.create_user(&new.guild_id.to_string(), &new.user.id.to_string(), &role_ids).await;
                    match res {
                        Ok(_) => {},
                        Err(e) => println!("Error creating user: {}", e)
                    }
                }
            },
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
    
    pub async fn guild_member_join(&self, ctx: Context, member: Member) {
        let user = self.db.get_user(&member.guild_id.to_string(), &member.user.id.to_string()).await;
        match user {
            Ok(user) => {
                match user {
                    Some(user) => {
                        for role_id in user.roles {
                            println!("User {} joined guild. Adding role {}", member.user.name, role_id);
                            member.clone().add_role(&ctx.http, RoleId::from(role_id.parse::<u64>().unwrap())).await.unwrap();
                        }
                    },
                    None => {}
                }
            },
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
