use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Role(String);

impl Role {
    pub async fn new(name: String, db: &PgPool) -> Option<Self> {
        sqlx::query!("SELECT name FROM role WHERE name = $1;", name)
            .fetch_optional(db)
            .await
            .unwrap()
            .map(|row| Self(row.name))
    }
}

pub async fn has_permission(_for: &Actor, on: Role, db: &PgPool) -> bool {
    sqlx::query!("SELECT * FROM user_role JOIN role ON user_role.role_id = role.id WHERE user_id = $1 AND role.name = $2;", _for.as_ref(), on.0)
            .fetch_optional(db)
            .await
            .unwrap()
            .is_some()
}

/// The unique identifier of a user, used to identify the policies
/// That apply to a user for a given resource.
pub struct Actor(Uuid);

impl AsRef<Uuid> for Actor {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

/// A generic trait for defining RBAC on a DB table
#[async_trait::async_trait]
pub trait Resource: Sized {
    type Authorized;
    async fn is_allowed(&self, db: &PgPool, role: Role, actor: &Actor) -> bool;

    async fn authorize_unchecked(self, db: &PgPool, actor: &Actor) -> Self::Authorized;

    /// Helper function which is literally
    /// ```
    /// if self.is_allowed() {
    ///     Some(self.authorize_unchecked())
    /// } else {
    ///     None
    /// }
    /// ```
    async fn authorize(self, db: &PgPool, role: Role, actor: Actor) -> Option<Self::Authorized>;
}
