use anyhow::Result;

use crate::domain::entity::{MailAddress, User, UserId};

pub trait UserRepository {
    fn save(&self, user: User) -> Result<()>;
    fn get_by_id(&self, id: UserId) -> Result<Option<User>>;
    fn get_by_mail(&self, id: MailAddress) -> Result<Option<User>>;
}
