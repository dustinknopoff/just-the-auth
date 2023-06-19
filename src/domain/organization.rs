use crate::{configuration::roles::ResourceConfiguration, rbac};
use rbac::{Resource, Policy, Actor, has_permission};
use sqlx::{PgPool};

pub struct Organization;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AuthorizedOrganization {
    id: i32,
    name: String,
    short_name: String,
    time_created: i32,
    time_updated: i32
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
