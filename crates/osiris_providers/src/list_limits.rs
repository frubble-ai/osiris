//! How far a provider's reads go: a list that fans out over several of its resources (how many it
//! reads at once, how many items it answers at most), and a text it downloads (a job's logs, of
//! which the end is kept). From `[providers.listing]` in the settings.

#[derive(Debug, Clone, Copy)]
pub struct ListLimits {
    /// Resources read at once (e.g. installations).
    pub concurrency: usize,
    /// Items answered at most; past it, the list stops reading.
    pub max_items: usize,
    /// The most of a downloaded text (a workflow job's logs) answered: its last bytes.
    pub max_log_bytes: usize,
}
