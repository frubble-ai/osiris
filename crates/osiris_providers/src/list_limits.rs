//! How far a list that fans out over several of the provider's resources goes: how many it reads
//! at once, and how many items it answers at most. From `[providers.listing]` in the settings.

#[derive(Debug, Clone, Copy)]
pub struct ListLimits {
    /// Resources read at once (e.g. installations).
    pub concurrency: usize,
    /// Items answered at most; past it, the list stops reading.
    pub max_items: usize,
}
