use pretty_assertions::assert_eq as diffed_eq;
use std::path::Path;
use std::process::Command;
use std::str::from_utf8;


const EMPTY_OUTPUT: &str = "";


pub fn assertRepositoryHasNoCommits(repositoryDir: &Path)
{
    assertFailedCommandErrorOutput(
        &["git", "log"],
        "fatal: your current branch 'master' does not have any commits yet\n",
        repositoryDir);
}

pub fn assertRepositoryLogIs(expectedOutput: &str, repositoryDir: &Path)
{
    assertCommandOutput(
        &["git", "log", "--pretty=Author: %an%nEmail: %ae%nSubject: %s", "--patch-with-stat"],
        expectedOutput,
        repositoryDir);
}

pub fn assertRepositoryStatusIs(expectedOutput: &str, repositoryDir: &Path)
{
    assertCommandOutput(&["git", "status", "--porcelain"], expectedOutput, repositoryDir);
}

pub fn assertRepositoryStatusIsEmpty(repositoryDir: &Path)
{
    assertRepositoryStatusIs(EMPTY_OUTPUT, repositoryDir);
}

fn assertCommandOutput(commandParts: &[&str], expectedOutput: &str, repositoryDir: &Path)
{
    let mut command = Command::new(commandParts[0]);
    command.args(&commandParts[1..]).current_dir(&repositoryDir);
    let output = command.output().unwrap();

    diffed_eq!(expectedOutput, from_utf8(&output.stdout).unwrap(),
               "\nExpected command output did not match actual.\nCommand: {:?}",
               command);
    assert_eq!(true, output.status.success(),
               "Command did not finish with success.\nCommand: {:?}\nExit status: {}",
               command, output.status);
    assert_eq!("", from_utf8(&output.stderr).unwrap(),
               "\nExpected command error output did not match actual.\nCommand: {:?}",
               command);
}

fn assertFailedCommandErrorOutput(commandParts: &[&str], expectedErrorOutput: &str, repositoryDir: &Path)
{
    let mut command = Command::new(commandParts[0]);
    command.args(&commandParts[1..]).current_dir(&repositoryDir);
    let output = command.output().unwrap();

    diffed_eq!(expectedErrorOutput, from_utf8(&output.stderr).unwrap(),
               "\nExpected command error output did not match actual.\nCommand: {:?}",
               command);
    assert_eq!(false, output.status.success(),
               "Command finished with success instead of failure.\nCommand: {:?}\nExit status: {}",
               command, output.status);
    assert_eq!("", from_utf8(&output.stdout).unwrap(),
               "\nExpected command output did not match actual.\nCommand: {:?}",
               command);
}