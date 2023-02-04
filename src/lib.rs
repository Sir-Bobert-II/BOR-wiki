use std::rc::Rc;

use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType};
use truncrate::*;
use wikipedia_api::*;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("wiki")
        .description("Get a summary of a topic from wikipedia.org")
        .dm_permission(true)
        .create_option(|option| {
            option
                .name("title")
                .description("The title to search wikipedia.org for")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

pub async fn run(title: String, max: usize) -> String {
    let page = Rc::new(match Page::search(&title).await {
        Ok(x) => x,
        Err(e) => return e.to_string(),
    });
    let url = page.get_url();
    let mut content = match page.get_summary().await {
        Ok(x) => x,
        Err(e) => return e.to_string(),
    };
    if content.len() >= max {
        content = format!("{}...", content.truncate_to_boundary(max));
    }

    format!("{content}\n{url}")
}
