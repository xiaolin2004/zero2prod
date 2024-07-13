use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use actix_web::{
    web::{self},
    HttpResponse,
};
use chrono::Utc;
use sqlx::types::chrono;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = % form.email,
        subscriber_name = % form.name,
    )
)]
pub async fn subscribe(pool: web::Data<PgPool>, form: web::Form<FormData>) -> HttpResponse {
    let new_subscriber = match form.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// 使用TryFrom Trait实现类型转换
impl TryFrom<FormData> for NewSubscriber {
    type Error = String;
    fn try_from(form: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(form.name)?;
        let email = SubscriberEmail::parse(form.email)?;
        Ok(NewSubscriber { name, email })
    }
}

pub fn parse_subscriber(form: web::Form<FormData>) -> Result<NewSubscriber, String> {
    let name = SubscriberName::parse(form.0.name)?;
    let email = SubscriberEmail::parse(form.0.email)?;
    Ok(NewSubscriber { name, email })
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(pool, new_subscriber)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id,email,name,subscribed_at,status)
        VALUES ($1,$2,$3,$4,'confirmed')
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
