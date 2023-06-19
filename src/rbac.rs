use sqlx::PgPool;

use crate::configuration::roles::{ResourceConfiguration, RoleAccess};

pub async fn has_permission(
    _for: &Actor,
    on: String,
    with: &Policy,
    db: &PgPool,
    config: &ResourceConfiguration,
) -> bool {
    let role_for_resource =
        sqlx::query_as!(RoleAccess, "SELECT * FROM role_access WHERE user_id = $1 AND resource = $2;", _for.as_ref(), on)
            .fetch_optional(db)
            .await
            .unwrap()
            .map(|row| row.role);
    if let Some(role) = role_for_resource {
        if let Some(permissions) = config.grants.get(&role) {
            return permissions
                .iter()
                .any(|permission| permission == with.as_ref());
        }
    } else if config.extensions.is_some() {
        let all_user_roles: Vec<RoleAccess> =  sqlx::query_as!(RoleAccess, "SELECT * FROM role_access WHERE user_id = $1;", _for.as_ref())
            .fetch_all(db)
            .await
            .unwrap();
        return all_user_roles.into_iter().any(|role_access| {
            if let Some(permissions) = config
                .extensions
                .as_ref()
                .unwrap()
                .get(&role_access.resource)
                .and_then(|ext| ext.get(&role_access.role))
                .and_then(|resource_role| config.grants.get(resource_role))
            {
                return permissions
                    .iter()
                    .any(|permission| permission == with.as_ref());
            }
            false
        });
    }
    false
}

/// The unique identifier of a user, used to identify the policies
/// That apply to a user for a given resource.
pub struct Actor(i32);

impl AsRef<i32> for Actor {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

/// A policy states
/// > Is the user granted permission for this action on the associated resource?
pub struct Policy(String);

impl AsRef<String> for Policy {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

/// A generic trait for defining RBAC on a DB table
#[async_trait::async_trait]
pub trait Resource: Sized {
    type Authorized;
    async fn is_allowed(
        &self,
        config: &ResourceConfiguration,
        db: &PgPool,
        policy: Policy,
        actor: &Actor,
    ) -> bool;

    async fn authorize_unchecked(self, db: &PgPool, actor: &Actor) -> Self::Authorized;

    /// Helper function which is literally
    /// ```
    /// if self.is_allowed() {
    ///     Some(self.authorize_unchecked())
    /// } else {
    ///     None
    /// }
    /// ```
    async fn authorize(
        self,
        config: &ResourceConfiguration,
        db: &PgPool,
        policy: Policy,
        actor: Actor,
    ) -> Option<Self::Authorized>;
}
