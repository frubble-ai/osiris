//! The list limits every live check builds its provider with.

use osiris_providers::ListLimits;

pub const LIMITS: ListLimits = ListLimits {
    concurrency: 4,
    max_items: 5000,
    max_log_bytes: 262_144,
};
