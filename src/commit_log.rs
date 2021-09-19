use crate::date_time::{LocalDateTime, ZERO_NANOSECONDS};
use crate::repository::Repository;

use chrono::TimeZone as _;

const INVALID_UTF8: &str = "<invalid UTF-8>";


pub struct CommitLog
{
    commits: Vec<CommitInfo>,
}

impl CommitLog
{
    pub fn new(repo: &Repository) -> Self
    {
        let mut newSelf = Self{commits: vec![]};
        newSelf.loadCommits(repo);
        newSelf
    }

    pub fn getCommits(&self) -> &[CommitInfo]
    {
        &self.commits
    }


    // private

    fn loadCommits(&mut self, repo: &Repository)
    {
        if repo.isEmpty() {
            return;
        }

        repo.iterateCommits(|commit| {
            let summary = getSummary(commit);
            let signature = commit.author();
            let date = chrono::Local.timestamp(signature.when().seconds(), ZERO_NANOSECONDS);
            let author = signature.name().unwrap_or(INVALID_UTF8).into();
            let email = signature.email().unwrap_or(INVALID_UTF8).into();
            let id = commit.id();
            self.commits.push(CommitInfo{id, summary, date, author, email});
        });
    }
}

fn getSummary(commit: &git2::Commit) -> String
{
    match commit.summary() {
        Some(summary) => summary.into(),
        None => getSummaryFromRaw(commit)
    }
}

fn getSummaryFromRaw(commit: &git2::Commit) -> String
{
    match commit.summary_bytes() {
        Some(bytes) => String::from_utf8_lossy(bytes).into(),
        None => "<UNKNOWN SUMMARY>".into()
    }
}

pub struct CommitInfo
{
    pub id: git2::Oid,
    pub summary: String,
    pub date: LocalDateTime,
    pub author: String,
    pub email: String,
}
