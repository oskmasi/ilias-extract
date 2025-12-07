use crate::submission::{IliasStudent, IliasSubmission, IliasTeam};
use anyhow::{Context, Result};
use calamine::{DataType, Reader, Xlsx};
use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::path::Path;

/// A row in the Excel spreadsheet
#[derive(Debug)]
struct IliasRow
{
    surname: String,
    name: String,
    login_name: String,
    timestamp: NaiveDateTime,
    team_id: i32,
    files: Vec<String>,
}

impl Into<IliasStudent> for &IliasRow
{
    fn into(self) -> IliasStudent
    {
        IliasStudent {
            surname: self.surname.clone(),
            name: self.name.clone(),
            login_name: self.login_name.clone(),
        }
    }
}

impl Into<Vec<IliasSubmission>> for &IliasRow
{
    fn into(self) -> Vec<IliasSubmission>
    {
        self.files.iter()
            .map(|str| IliasSubmission { timestamp: self.timestamp.clone(), file_name: str.to_string() })
            .collect()
    }
}

impl Into<IliasTeam> for &IliasRow
{
    /// Turns this row into a team with one student already present
    fn into(self) -> IliasTeam {
        IliasTeam {
            id: self.team_id.clone(),
            submissions: self.into(),
            members: vec![self.into()]
        }
    }
}

/// The Excel file in our archive which contains details about
/// all submissions and teams
#[derive(Debug)]
pub struct IliasMetadataFile
{
    /// All rows in the spreadsheet
    records: Vec<IliasRow>,
}

impl IliasMetadataFile
{
    /// Parses the file at the given path
    pub fn new(path: &Path) -> Result<IliasMetadataFile>
    {
        let mut workbook: Xlsx<_> = calamine::open_workbook(path).with_context(|| "Error in Excel workbook containing student information")?;
        let spreadsheet = workbook.worksheet_range_at(0).with_context(|| "Spreadsheet is empty")??;
        let mut records: Vec<IliasRow> = Vec::new();
        for row in spreadsheet.rows().skip(1)
        {
            let mut files: Vec<String> = Vec::new();
            for i in 5..row.len()
            {
                if let Some(file_name) = row[i].get_string()
                {
                    files.push(file_name.to_string());
                }
            }
            let metadata = IliasRow
            {
                surname:    row[0].get_string().unwrap_or_default().to_string(),
                name:       row[1].get_string().unwrap_or_default().to_string(),
                login_name: row[2].get_string().unwrap_or_default().to_string(),
                timestamp:  NaiveDateTime::parse_from_str(row[3].get_string().unwrap_or_default(), "%Y-%m-%d %H:%M:%S")?,
                team_id:    row[4].get_float().unwrap_or_default() as i32, // I do not know why this is a float??
                files
            };
            records.push(metadata);
        }
        Ok(IliasMetadataFile { records } )
    }
}

impl Into<HashMap<i32, IliasTeam>> for IliasMetadataFile
{
    /// Generates the team structure from this file
    fn into(self) -> HashMap<i32, IliasTeam>
    {
        let mut teams: HashMap<i32, IliasTeam> = HashMap::new();
        for record in self.records
        {
            if !teams.contains_key(&record.team_id)
            {
                // Team is not hashed yet, create it from this row with one member already present
                teams.insert(record.team_id.clone(), (&record).into());
            } else
            {
                // Team already exists, just add a member
                teams.get_mut(&record.team_id).unwrap().members.push((&record).into());
            }
        }
        teams
    }
}