//! A batch from the database as the API's list.

use osiris_api_models::list::List;
use osiris_domain::Page;

pub fn list<T, U: From<T>>(page: Page<T>) -> List<U> {
    List {
        data: page.items.into_iter().map(U::from).collect(),
        next_cursor: page.next,
        previous_cursor: page.previous,
    }
}
