use anyhow::{anyhow, Result};
use clap::ArgMatches;

use crate::media::handle;

pub fn tags_from_matches(matches: &ArgMatches) -> Vec<&String> {
    matches
        .get_many::<String>("TAG")
        .map_or_else(Vec::new, Iterator::collect)
}

pub fn handle_from_matches(matches: &ArgMatches) -> Result<Option<handle::Handle>> {
    let user_input = match matches.try_get_one::<String>("IDENTIFIER")? {
        Some(i) => i.to_string(),
        None => return Ok(None),
    };

    let identifier = match matches.try_get_one::<u16>("YEAR")? {
        Some(year) => format!("{user_input} ({year})"),
        None => user_input,
    };

    Ok(Some(handle::Handle::from_user_input(identifier.as_str())))
}

pub fn note_from_matches(matches: &ArgMatches) -> Result<Option<String>> {
    let note_match = matches.try_get_one::<String>("NOTE")?;
    if let Some(note) = note_match {
        if note.contains('\n') {
            return Err(anyhow!("note should be a single line"));
        }
    }

    Ok(note_match.map(Into::into))
}
