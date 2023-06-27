use chrono::{DateTime, Utc};
use cloudflare::endpoints::account::Settings;

/// Cloudflare Accounts
/// An Account is the root object which owns other resources such as zones, load balancers and billing details.
/// https://api.cloudflare.com/#accounts-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Account {
    /// Account identifier tag.
    pub id: String,
    /// Account name
    pub name: String,
    /// Account Settings
    pub settings: Option<Settings>,
    /// describes when the account was created
    pub created_on: Option<DateTime<Utc>>,
}
