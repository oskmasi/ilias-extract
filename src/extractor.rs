use crate::submission::IliasTeam;
use crate::IliasConfiguration;
use anyhow::{anyhow, bail, Context, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io;
use std::io::Write;
use std::path::PathBuf;
use zip::ZipArchive;

/// Extracts the zip archive
pub struct IliasExtractor
{
    /// Contents from the Excel spreadsheet
    teams: HashMap<i32, IliasTeam>,

    /// The archive itself
    archive: ZipArchive<File>,

    /// An index of the archive mapping all paths to zip indices
    archive_index: HashMap<PathBuf, usize>,
}

impl IliasExtractor
{
    /// Creates a new extractor instance
    pub fn new(teams: HashMap<i32, IliasTeam>, archive: ZipArchive<File>, archive_index: HashMap<PathBuf, usize>) -> Self
    {
        IliasExtractor { teams, archive, archive_index }
    }

    /// Performs the action
    pub fn extract(mut self, configuration: &IliasConfiguration) -> Result<()>
    {
        // Target folders for each team
        let mut folders: HashMap<i32, PathBuf> = HashMap::new();

        // Create folder structure for all teams first
        for (team_id, team) in self.teams.iter()
        {
            let mut usernames = team.members.iter().map(|member| member.username()).collect::<Result<Vec<_>>>()?;
            usernames.sort();
            let folder_name = usernames.join("_");
            let sub_path = configuration.output.join(&folder_name);
            if sub_path.exists()
            {
                if !sub_path.is_dir()
                {
                    bail!("{} is not a directory", sub_path.display());
                }
            } else
            {
                create_dir_all(&sub_path).with_context(|| format!("Unable to create folder {}", sub_path.display()))?;
            }

            if configuration.summary
            {
                let summary_file = sub_path.join("summary.md");
                let markdown = team.generate_summary();
                if configuration.verbose
                {
                    println!("Team {}: Creating summary file at {}", team_id, summary_file.display());
                }
                File::create(&summary_file)?.write_all(markdown.as_bytes())?;
            }

            folders.insert(team_id.clone(), sub_path);
        }

        // Then iterate over contents of zip file
        let submission_regex = Regex::new(r"(?m)^([^/]+)/Abgaben/Team ([0-9]+)/([^/]+)/([^.]+).([a-zA-Z]+)$")?;
        for (path, index) in self.archive_index
        {
            let mut file = self.archive.by_index(index)?;
            let path_str = path.to_str().unwrap();
            if let Some(captures) = submission_regex.captures(path_str)
            {
                // This path matches the regex
                let (_, [_, team_str, _, name, ext]) = captures.extract();
                let team_id = team_str.parse::<i32>()?;
                let target_folder = folders.get(&team_id).ok_or_else(|| anyhow!("Unable to find entry for team: {}", &team_id))?;
                let target_path = target_folder.join(format!("{}.{}", name, ext));
                if configuration.verbose
                {
                    println!("Team {}: Extracting {} to {}", team_id, path_str, target_path.display());
                }
                let mut target_file = match configuration.overwrite
                {
                    true => File::create(&target_path),
                    false => File::create_new(&target_path)
                }.with_context(|| format!("Unable to create new file: {}, consider using --purge or --overwrite", &target_path.display()))?;
                io::copy(&mut file, &mut target_file)?;
            }
        }
        Ok(())
    }
}