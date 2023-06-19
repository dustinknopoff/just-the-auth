use crate::{configuration::roles::ResourceConfiguration, rbac};
use chrono::{DateTime, Utc};
use rbac::{Resource, Policy, Actor, has_permission};
use sqlx::{PgPool};
use uuid::Uuid;

pub struct Organization;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AuthorizedOrganization {
    id: Uuid,
    name: String,
    short_name: String,
    time_created: DateTime<Utc>,
    time_updated: DateTime<Utc>
}

#[async_trait::async_trait]
impl Resource for Organization {
    type Authorized = AuthorizedOrganization;
    async fn is_allowed(
        &self,
        config: &ResourceConfiguration,
        db: &PgPool,
        policy: Policy,
        actor: &Actor,
    ) -> bool {
        has_permission(actor, String::from("organization"), &policy, db, config).await
    }

    async fn authorize_unchecked(self, db: &PgPool, actor: &Actor) -> Self::Authorized {
        sqlx::query_as!(AuthorizedOrganization, "SELECT o.* FROM organization o, user_organization uo WHERE uo.organization_id = o.id AND uo.user_id = $1;", actor.as_ref())
        .fetch_one(db)
        .await
        .unwrap()
    }

    async fn authorize(
        self,
        config: &ResourceConfiguration,
        db: &PgPool,
        policy: Policy,
        actor: Actor,
    ) -> Option<Self::Authorized> {
        if self.is_allowed(config, db, policy, &actor).await {
            Some(self.authorize_unchecked(db, &actor).await)
        } else {
            None
        }
    }
}
