use anyhow::*;
use cargo_metadata::MetadataCommand;
use cradle::prelude::*;
use question::{Answer, Question};
use std::env;
use structopt::StructOpt;
use tempfile::tempdir;

#[derive(StructOpt)]
struct Arguments {
  revision: String,
}

fn main() -> Result<()> {
  let arguments = Arguments::from_args();
  let tempdir = tempdir()?;
  let StdoutTrimmed(repo_url) = cmd!(%"git remote get-url origin");
  let current_dir = env::current_dir()?;
  let crate_name = current_dir
    .file_name()
    .ok_or(anyhow!("current dir has no file name"))?
    .to_str()
    .ok_or(anyhow!("current dir not valid utf-8"))?;
  (
    CurrentDir(tempdir.path()),
    "git",
    "clone",
    &repo_url,
    crate_name,
  )
    .run_unit();
  env::set_current_dir(tempdir.path().join(crate_name))?;

  (
    "git",
    "merge-base",
    "--is-ancestor",
    &arguments.revision,
    "master",
  )
    .run_unit();
  ("git", "checkout", &arguments.revision).run_unit();

  ("cargo", "publish", "--dry-run").run_unit();

  let metadata = MetadataCommand::new().exec()?;

  let version = metadata
    .packages
    .into_iter()
    .filter(|package| package.name == crate_name)
    .next()
    .ok_or(anyhow!(
      "package '{}' not found in cargo metadata",
      crate_name
    ))?
    .version;

  let tag_name = format!("v{}", version);

  eprintln!("crate name: {}", crate_name);
  eprintln!("repo: {}", repo_url);
  eprintln!("revision: {} (merged into master)", arguments.revision);
  eprintln!("tag: {}", tag_name);
  let answer = Question::new("Continue?")
    .default(Answer::NO)
    .show_defaults()
    .confirm();

  if let Answer::YES = answer {
    (LogCommand, "cargo", "publish").run_unit();
    (LogCommand, "git", "tag", &tag_name).run_unit();
    (LogCommand, "git", "push", "origin", tag_name).run_unit();
  }
  Ok(())
}
