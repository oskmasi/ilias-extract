use anyhow::bail;
use anyhow::Result;
use chrono::NaiveDateTime;
use regex::Regex;

/// A team of students
#[derive(Default, Debug)]
pub struct IliasTeam
{
    /// Numerical id given by Ilias
    pub id: i32,

    /// All their submitted files
    pub submissions: Vec<IliasSubmission>,

    /// All team members
    pub members: Vec<IliasStudent>
}

impl IliasTeam
{
    /// Creates a new empty team of an id
    pub fn new(id: i32) -> IliasTeam
    {
        IliasTeam {
            id,..Default::default()
        }
    }

    /// Generates a Markdown summary of the team members and their submissions
    pub fn generate_summary(&self) -> String
    {
        let header = format!("Team {}", self.id);
        let member_list = self.members.iter()
            .map(|member| format!("- {} {} ({})", member.name, member.surname, member.login_name))
            .collect::<Vec<_>>().join("\n");
        let file_list = self.submissions.iter()
            .map(|submission| format!("- {} ({})", submission.file_name, submission.timestamp))
            .collect::<Vec<_>>().join("\n");
        format!("# {}\n## Members:\n{}\n## Submissions:\n{}", header, member_list, file_list)
    }
}

/// A student (member of a team)
#[derive(Debug)]
pub struct IliasStudent
{
    pub surname: String,
    pub name: String,
    pub login_name: String,
}

impl IliasStudent
{
    /// The username part (e.g. abc123) of the login name abc123@foo_bar
    pub fn username(&self) -> Result<&str>
    {
        let id_pattern = Regex::new(r"^([^@]+)")?;
        if let Some(caps) = id_pattern.captures(self.login_name.as_ref())
        {
            Ok(caps.get(1).unwrap().as_str())
        } else
        {
            bail!("Unrecognized login name (expected id123@foo_bar): {}", self.login_name)
        }
    }
}

#[derive(Debug)]
pub struct IliasSubmission
{
    /// Date and time of submission
    pub timestamp: NaiveDateTime,

    /// Title of the submission (file name without path)
    pub file_name: String,
}