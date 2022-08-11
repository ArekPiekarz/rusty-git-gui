use crate::date_time::makeDateTime;
use crate::repository::Repository;

use time::OffsetDateTime;

const INVALID_UTF8: &str = "<invalid UTF-8>";


pub(crate) struct CommitLog
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

    pub fn getCommit(&self, row: usize) -> Option<&CommitInfo>
    {
        self.commits.get(row)
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
            let date = makeDateTime(&commit.time());
            let author = signature.name().unwrap_or(INVALID_UTF8).into();
            let email = signature.email().unwrap_or(INVALID_UTF8).into();
            let id = commit.id();
            self.commits.push(CommitInfo{id, summary, date, author, email});
        });
    }
}

fn getSummary(commit: &git2::Commit) -> String
{
    // Only consider using git2::Commit::summary() again if this bug in libgit2 is fixed, which sometimes causes
    // the output to be too long: https://github.com/libgit2/libgit2/issues/6065
    String::from_utf8_lossy(commit.message_bytes()).lines().next().unwrap_or_default().trim().into()
}

pub(crate) struct CommitInfo
{
    pub id: git2::Oid,
    pub summary: String,
    pub date: OffsetDateTime,
    pub author: String,
    pub email: String,
}
