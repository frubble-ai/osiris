//! The client's callback URL with `connected_account_id` and `status` added.

use osiris_domain::ConnectionStatus;
use url::Url;

pub fn back_to(callback_url: &str, id: &str, status: ConnectionStatus) -> String {
    match Url::parse(callback_url) {
        Ok(mut url) => {
            url.query_pairs_mut()
                .append_pair("connected_account_id", id)
                .append_pair("status", status.as_str());
            url.to_string()
        }
        Err(_) => callback_url.to_owned(),
    }
}
