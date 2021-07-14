use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use super::{MailAddress, UserId};

#[derive(Clone, Debug, Getters, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    mail: MailAddress,
}

impl User {
    pub fn new(mail: MailAddress) -> Self {
        Self {
            id: UserId::default(),
            mail,
        }
    }

    pub fn rebuild(id: UserId, mail: MailAddress) -> Self {
        Self { id, mail }
    }
}
