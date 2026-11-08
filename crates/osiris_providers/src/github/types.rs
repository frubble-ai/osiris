//! GitHub's resources in our terms, one file each: the shapes osiris's GitHub actions answer with,
//! the fields `OpenConnector`'s GitHub output schemas name (v1.9.4, Apache-2.0), typed, with
//! GitHub's numeric ids as strings and each page on GitHub as `url`.

pub mod acknowledgement;
pub mod artifact;
pub mod artifact_download;
pub mod branch;
pub mod check_run;
pub mod collaborator;
pub mod collaborator_addition;
pub mod combined_status;
pub mod commit;
pub mod commit_comment;
pub mod commit_comparison;
pub mod commit_status;
pub mod content;
pub mod contributor;
pub mod event;
pub mod fork_sync;
pub mod git_ref;
pub mod installation;
pub mod issue;
pub mod issue_comment;
pub mod issue_event;
<<<<<<< HEAD
pub mod issue_lock;
pub mod issue_page;
=======
pub mod job_logs;
>>>>>>> port/github-checks_workflows
pub mod label;
pub mod language;
pub mod merge_result;
pub mod merged_check;
pub mod milestone;
pub mod public_user;
pub mod pull_request;
pub mod pull_request_branch_update;
pub mod pull_request_file;
pub mod reaction;
pub mod release;
pub mod release_asset;
pub mod repository;
pub mod repository_permission;
pub mod requested_reviewers;
pub mod review;
pub mod review_comment;
pub mod review_request;
pub mod search;
<<<<<<< HEAD
pub mod search_issue_results;
=======
pub mod search_results;
pub mod starred_check;
>>>>>>> port/github-releases_activity_search
pub mod tag;
pub mod team;
<<<<<<< HEAD
pub mod topics;
=======
pub mod timeline_event;
>>>>>>> port/github-issues
pub mod user;
pub mod workflow;
pub mod workflow_job;
pub mod workflow_run;

pub use acknowledgement::GithubAcknowledgement;
pub use artifact::GithubArtifact;
pub use artifact_download::GithubArtifactDownload;
pub use branch::GithubBranch;
pub use check_run::GithubCheckRun;
pub use collaborator::{GithubCollaborator, GithubPermissions};
pub use collaborator_addition::{GithubCollaboratorAddition, GithubRepositoryInvitation};
pub use combined_status::GithubCombinedStatus;
pub use commit::{GithubCommit, GithubCommitFile, GithubCommitStats};
pub use commit_comment::GithubCommitComment;
pub use commit_comparison::GithubCommitComparison;
pub use commit_status::GithubCommitStatus;
pub use content::{
    GithubContentChange, GithubContentCommit, GithubContentEntry, GithubFileContent,
};
pub use contributor::GithubContributor;
pub use event::{GithubEvent, GithubEventRepository};
pub use fork_sync::GithubForkSync;
pub use git_ref::GithubGitRef;
pub use installation::GithubInstallation;
pub use issue::GithubIssue;
pub use issue_comment::GithubIssueComment;
pub use issue_event::GithubIssueEvent;
<<<<<<< HEAD
pub use issue_lock::GithubIssueLock;
pub use issue_page::GithubIssuePage;
=======
pub use job_logs::GithubJobLogs;
>>>>>>> port/github-checks_workflows
pub use label::GithubLabel;
pub use language::GithubLanguage;
pub use merge_result::GithubMergeResult;
pub use merged_check::GithubMergedCheck;
pub use milestone::GithubMilestone;
pub use public_user::GithubPublicUser;
pub use pull_request::{GithubPullRequest, GithubPullRequestBranch};
pub use pull_request_branch_update::GithubPullRequestBranchUpdate;
pub use pull_request_file::GithubPullRequestFile;
pub use reaction::GithubReaction;
pub use release::{GithubRelease, GithubReleaseNotes};
pub use release_asset::GithubReleaseAsset;
pub use repository::GithubRepository;
pub use repository_permission::GithubRepositoryPermission;
pub use requested_reviewers::GithubRequestedReviewers;
pub use review::GithubReview;
pub use review_comment::GithubReviewComment;
pub use review_request::GithubReviewRequest;
pub use search::{
    GithubSearchCode, GithubSearchCodeRepository, GithubSearchCommit, GithubSearchCommitRepository,
    GithubSearchIssue, GithubSearchLabel, GithubSearchRepository, GithubSearchTopic,
    GithubSearchUser,
};
<<<<<<< HEAD
pub use search_issue_results::GithubSearchIssueResults;
=======
pub use search_results::{
    GithubCodeSearchResults, GithubCommitSearchResults, GithubLabelSearchResults,
    GithubRepositorySearchResults, GithubTopicSearchResults, GithubUserSearchResults,
};
pub use starred_check::GithubStarredCheck;
>>>>>>> port/github-releases_activity_search
pub use tag::GithubTag;
pub use team::GithubTeam;
<<<<<<< HEAD
pub use topics::GithubTopics;
=======
pub use timeline_event::GithubTimelineEvent;
>>>>>>> port/github-issues
pub use user::{GithubCurrentUser, GithubUser};
pub use workflow::GithubWorkflow;
pub use workflow_job::{GithubWorkflowJob, GithubWorkflowStep};
pub use workflow_run::GithubWorkflowRun;
