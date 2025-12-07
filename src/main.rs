mod submission;
mod excel;
mod extractor;

use crate::excel::IliasMetadataFile;
use crate::extractor::IliasExtractor;
use crate::submission::IliasTeam;
use anyhow::{bail, Context, Result};
use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{remove_dir_all, File};
use std::io;
use std::path::PathBuf;
use zip::ZipArchive;

#[derive(Parser, Debug)]
#[command(name="ilias-extract", author = "osmasi", about = "Extracts zip files containing student submissions from Ilias and organizes them")]
pub struct IliasConfiguration
{
    /// Path to the Ilias zip file
    #[arg(value_name = "Input file", help = "Path to the zip file downloaded from Ilias")]
    input: PathBuf,

    #[arg(value_name = "Target directory", help = "Output folder where all content is placed in")]
    output: PathBuf,

    #[arg(short, long, help = "Generates a summary Markdown file for every student")]
    summary: bool,

    #[arg(short, long, help = "Enables overwriting of already existing files")]
    overwrite: bool,

    #[arg(long, help = "Clears the entire target directory before unzipping (caution!)")]
    purge: bool,

    #[arg(short, long, help = "Enables verbose output")]
    verbose: bool
}

fn main() -> Result<()>
{
    let args = IliasConfiguration::parse();
    if args.output.exists() && !args.output.is_dir()
    {
        bail!("Output path is not a directory!");
    }
    if args.purge && args.output.is_dir()
    {
        remove_dir_all(&args.output).with_context(|| format!("Unable to purge output directory: {}", &args.output.display()))?;
    }

    let metadata_regex = Regex::new(r"(?m)^([^/]+)/([^/]+).xlsx$")?;
    let zip_path = args.input.clone();
    let zip_file = File::open(&zip_path).with_context(|| format!("Unable to open file: {}", &zip_path.display()))?;
    let mut zip = ZipArchive::new(zip_file).with_context(|| format!("Unable to open zip archive: {}", &zip_path.display()))?;

    // Maps path names in the zip archive to their ZipFile index
    let mut content: HashMap<PathBuf, usize> = HashMap::new();
    // Metadata file containing all submission records
    let mut metadata_file: Option<IliasMetadataFile> = None;

    // Scan the archive and put all paths with their indices in the hash map
    // Also search for the metadata spreadsheet
    for i in 0..zip.len()
    {
        let mut file = zip.by_index(i)?;
        let path = file.enclosed_name().with_context(|| format!("Illegal path in zip archive (zip slip?): {}", &file.mangled_name().display()))?;
        let path_str = path.to_str().unwrap();
        if metadata_regex.is_match(path_str)
        {
            // We found the metadata Excel file
            if metadata_file.is_some()
            {
                bail!("Multiple metadata files matched: {}", &path_str);
            }
            let mut temp_file = tempfile::Builder::new().tempfile().with_context(|| format!("Unable to create tempfile: {}", &path_str))?;
            io::copy(&mut file, &mut temp_file).with_context(|| format!("Unable to unzip Excel file: {}", &path_str))?;
            metadata_file = IliasMetadataFile::new(temp_file.path())?.into();
            continue;
        }
        content.insert(path, i);
    }

    if metadata_file.is_none()
    {
        bail!("Unable to find metadata file (xlsl file) in archive");
    }

    let teams: HashMap<i32, IliasTeam> = metadata_file.unwrap().into();
    let extractor = IliasExtractor::new(teams, zip, content);
    extractor.extract(&args)
}
