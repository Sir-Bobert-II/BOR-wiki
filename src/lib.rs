use std::{sync::{Arc, Mutex}};

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
    let page = Mutex::new(match Page::search(&title).await {
        Ok(x) => x,
        Err(e) => return e.to_string(),
    });
    let url = page.lock().unwrap().get_url();
    let mut content = match page.lock().unwrap().get_summary().await{
        Ok(x) => x,
        Err(e) => return e.to_string(),
    };
    if content.len() >= max {
        content = format!("{}...", content.truncate_to_boundary(max));
    }

    format!("{content}\n{url}")
}
