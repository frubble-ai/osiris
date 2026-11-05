//! Cursor pagination: a list is answered in batches ("pages") of `limit` items. Each batch says
//! the cursor of the next (`after`) and the previous (`before`); a cursor is an item's id, and
//! batches are ordered by (`created_at`, id).

/// Where a page starts, and how long it is: the caller has already bounded `limit`.
#[derive(Debug, Clone, Default)]
pub struct Cursor {
    pub after: Option<String>,
    pub before: Option<String>,
    pub limit: i64,
}

impl Cursor {
    pub fn limit(&self) -> i64 {
        self.limit.max(1)
    }
}

#[derive(Debug, Clone)]
pub struct Page<T> {
    pub items: Vec<T>,
    /// The cursor of the next page (`after`), if there is one.
    pub next: Option<String>,
    /// The cursor of the previous page (`before`), if there is one.
    pub previous: Option<String>,
}

impl<T> Page<T> {
    /// `items` was fetched one past the limit, forwards, or backwards when `before` is set.
    pub fn from_overfetch(mut items: Vec<T>, cursor: &Cursor, id: impl Fn(&T) -> String) -> Self {
        let limit = usize::try_from(cursor.limit()).unwrap_or(usize::MAX);
        let more = items.len() > limit;
        items.truncate(limit);
        if cursor.before.is_some() {
            items.reverse();
            Self {
                previous: if more { items.first().map(&id) } else { None },
                next: items.last().map(&id),
                items,
            }
        } else {
            Self {
                previous: if cursor.after.is_some() {
                    items.first().map(&id)
                } else {
                    None
                },
                next: if more { items.last().map(&id) } else { None },
                items,
            }
        }
    }
}
