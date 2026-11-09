//! What an issue event is about (its label, milestone or rename), as GitHub answers it.

use crate::github::types::{GithubEventLabel, GithubEventMilestone, GithubEventRename};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EventLabelAnswer {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub color: String,
}

#[derive(Deserialize)]
pub struct EventMilestoneAnswer {
    #[serde(default)]
    pub title: String,
}

#[derive(Deserialize)]
pub struct EventRenameAnswer {
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
}

/// The issue an event in a repository's list is about.
#[derive(Deserialize)]
pub struct EventIssueAnswer {
    pub number: i64,
}

impl From<EventLabelAnswer> for GithubEventLabel {
    fn from(answer: EventLabelAnswer) -> Self {
        Self {
            name: answer.name,
            color: answer.color,
        }
    }
}

impl From<EventMilestoneAnswer> for GithubEventMilestone {
    fn from(answer: EventMilestoneAnswer) -> Self {
        Self {
            title: answer.title,
        }
    }
}

impl From<EventRenameAnswer> for GithubEventRename {
    fn from(answer: EventRenameAnswer) -> Self {
        Self {
            from: answer.from,
            to: answer.to,
        }
    }
}
