use crate::date_time::ZERO_NANOSECONDS;
use crate::diff_formatter::{DiffFormatter, FormattedDiff, LineFormat};

use chrono::TimeZone as _;


pub fn formatCommitDiff(commit: &git2::Commit, diff: &git2::Diff) -> FormattedDiff
{
    let commitSummary = makeCommitSummary(&commit);
    let mut diffFormatter = DiffFormatter::newForCommit();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| diffFormatter.format(&line)).unwrap();
    let formattedDiff = diffFormatter.takeOutput();
    let text = commitSummary.text + &formattedDiff.text;
    let mut lineFormats = commitSummary.lineFormats;
    lineFormats.extend(formattedDiff.lineFormats);
    FormattedDiff{text, lineFormats}
}


// private

fn makeCommitSummary(commit: &git2::Commit) -> FormattedDiff
{
    let text = format!(
        "Commit: {}\nAuthor: {} <{}>\nDate:   {}\n\n{}\n",
        commit.id(),
        commit.author().name().unwrap(),
        commit.author().email().unwrap(),
        chrono::Local.timestamp(commit.time().seconds(), ZERO_NANOSECONDS).to_rfc2822(),
        tabulateCommitMessage(&getMessage(commit)));
    let lineFormats = vec![LineFormat::TopHeader; text.lines().count()];
    FormattedDiff{text, lineFormats}
}

fn getMessage(commit: &git2::Commit) -> String
{
    String::from_utf8_lossy(commit.message_bytes()).into()
}

fn tabulateCommitMessage(message: &str) -> String
{
    let mut result = String::new();
    for line in message.lines() {
        result.push_str(&format!("    {}\n", line));
    }
    result
}
