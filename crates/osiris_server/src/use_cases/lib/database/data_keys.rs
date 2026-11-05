//! `data_key`: each tenant's data key, made on first use, kept unwrapped in memory once read. One
//! operation per module.

pub mod cipher_for;
pub mod load_ciphers;

#[cfg(test)]
mod tests;
