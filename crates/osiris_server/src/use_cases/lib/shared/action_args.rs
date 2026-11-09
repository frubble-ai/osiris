//! What every action's use case takes: the connection it runs through, and its input.

pub struct ActionArgs<'a, I> {
    /// The connection to run through.
    pub id: &'a str,
    /// The action's input, as its route read it.
    pub input: &'a I,
}
