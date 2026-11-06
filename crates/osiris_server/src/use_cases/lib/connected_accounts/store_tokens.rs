//! Saves new tokens, encrypted with the user's data key, together with their generation number and
//! the refresh task scheduled for them. Everything is written in one statement, so tokens are never
//! saved without their refresh task. The failed-refresh count is reset.
//!
//! The write only happens if the row is still on the previous generation (and, when `from` is
//! given, still in that status). Otherwise someone else changed the connection first, and the
//! write fails with a conflict.

use super::{NewTokens, ScheduledRefresh};
use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::errors::db;
use crate::use_cases::lib::database::rows::encrypt;
use osiris_common::{crypto::Cipher, secret::ExposeSecret};
use osiris_domain::ConnectionStatus;
use osiris_domain::DomainError;
use sqlx::PgConnection;
use time::OffsetDateTime;

pub struct TokenWrite<'a> {
    pub id: &'a str,
    pub tokens: &'a NewTokens,
    /// The generation number of these tokens. The row must currently be on the number before.
    pub generation: i64,
    pub refresh: Option<&'a ScheduledRefresh>,
    pub status: ConnectionStatus,
    /// The status the row must be in, when that matters (for example, `pending` when finishing a
    /// consent).
    pub from: Option<ConnectionStatus>,
}

/// What the database stamped on the row while writing.
pub struct Stamped {
    pub updated_at: OffsetDateTime,
    pub refresh_token_issued_at: Option<OffsetDateTime>,
}

pub async fn store_tokens(
    connection: &mut PgConnection,
    cipher: &Cipher,
    write: TokenWrite<'_>,
) -> UseCaseResult<Stamped> {
    let access = encrypt(cipher, write.tokens.access_token.expose_secret())?;
    let refresh_token = write
        .tokens
        .refresh_token
        .as_ref()
        .map(|token| encrypt(cipher, token.expose_secret()))
        .transpose()?;
    let written = sqlx::query!(
        "UPDATE connected_account SET
             access_token = $2,
             refresh_token = COALESCE($3, refresh_token),
             refresh_token_issued_at = CASE WHEN $3::bytea IS NULL THEN refresh_token_issued_at ELSE now() END,
             access_token_expires_at = $4,
             scope = COALESCE($5, scope),
             status = $6,
             token_generation = $7::bigint,
             refresh_task = $8,
             refresh_task_at = $9,
             last_refresh_error = NULL,
             refresh_failures = 0,
             updated_at = now()
         WHERE id = $1 AND token_generation = $7::bigint - 1 AND ($10::text IS NULL OR status = $10)
         RETURNING updated_at, refresh_token_issued_at",
        write.id,
        access,
        refresh_token,
        write.tokens.access_token_expires_at,
        write.tokens.scope,
        write.status.as_str(),
        write.generation,
        write.refresh.map(|refresh| refresh.name.as_str()),
        write.refresh.map(|refresh| refresh.at),
        write.from.map(ConnectionStatus::as_str),
    )
    .fetch_optional(connection)
    .await
    .map_err(db)?;
    let Some(written) = written else {
        return Err(DomainError::Conflict("the connection moved on meanwhile".into()).into());
    };
    Ok(Stamped {
        updated_at: written.updated_at,
        refresh_token_issued_at: written.refresh_token_issued_at,
    })
}
