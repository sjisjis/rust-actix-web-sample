use anyhow::Result;
use chrono::{DateTime, Utc};
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool, Done};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRequest {
    pub name: String,
    pub mailadress: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateRequest {
    pub password: String,
}

#[derive(Serialize, FromRow, Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub mailadress: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub async fn find_all(pool: &MySqlPool) -> Result<Vec<User>> {
        let mut users = vec![];
        let recs = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, mailadress, password, created_at, updated_at, deleted_at
            FROM user
            ORDER BY id
            "#,
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            users.push(User {
                id: rec.id,
                name: rec.name,
                mailadress: rec.mailadress,
                password: rec.password,
                created_at: rec.created_at,
                updated_at: rec.updated_at,
                deleted_at: rec.deleted_at,
            });
        }

        Ok(users)
    }

    pub async fn find_by_id(id: i32, pool: &MySqlPool) -> Result<User> {
        let rec = sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = ?;")
            .bind(id)
            .fetch_one(&*pool)
            .await?;

        Ok(User {
            id: rec.id,
            name: rec.name,
            mailadress: rec.mailadress,
            password: rec.password,
            created_at: rec.created_at,
            updated_at: rec.updated_at,
            deleted_at: rec.deleted_at,
        })
    }

    pub async fn create(user: UserRequest, pool: &MySqlPool) -> Result<u64> {
        let mut tx = pool.begin().await?;
        info!("{}", format!("mysql://{:?}", user));
        let r = sqlx::query("INSERT INTO user (name, mailadress, password, created_at, updated_at) VALUES (?, ?, ?, now(), now());")
        .bind(&user.name)
        .bind(&user.mailadress)
        .bind(&user.password)
        .execute(&mut tx)
        .await?
        .last_insert_id();
        println!("{:?}", r);
        tx.commit().await?;
        Ok(r)
    }

    pub async fn update(id: u32, user: UpdateRequest, pool: &MySqlPool) -> Result<bool> {
        let mut tx = pool.begin().await?;
        let user = sqlx::query("UPDATE user SET password = ?, updated_at = now() WHERE id = ?;")
            .bind(&user.password)
            .bind(id)
            .execute(&mut tx)
            .await?
            .rows_affected();
        tx.commit().await?;
        println!("{:?}", user);
        Ok(user > 0)
    }
 
    pub async fn delete_put(id: u32, pool: &MySqlPool) -> Result<bool> {
        let mut tx = pool.begin().await?;
        let user = sqlx::query("UPDATE user SET deleted_at = now() WHERE id = ?;")
            .bind(id)
            .execute(&mut tx)
            .await?
            .rows_affected();
        tx.commit().await?;
        println!("{:?}", user);
        Ok(user > 0)
    }
   
    pub async fn delete(id: u32, pool: &MySqlPool) -> Result<u64> {
        let mut tx = pool.begin().await?;
        let d = sqlx::query("DELETE FROM user WHERE id = ?")
            .bind(id)
            .execute(&mut tx)
            .await?
            .rows_affected();

        tx.commit().await?;
        Ok(d)
    }
}