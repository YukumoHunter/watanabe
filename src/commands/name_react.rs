use std::collections::HashMap;

use serenity::{client::Context, framework::standard::CommandResult, model::channel::Message};

pub async fn name_react(ctx: &Context, msg: &Message) -> CommandResult {
    // Has to be in a guild for custom emoji
    if let Some(id) = msg.guild_id {
        let mut emojis = id.emojis(&ctx.http).await?;

        // TODO: filter emoji using user defined group in database instead
        let reaction_map = HashMap::from([
            // (emote name, trigger)
            ("pete", "pete"),
            ("sklarp", "sklarp"),
            ("bib", "bib"),
            ("figohane", "figo"),
            ("true", "true")
        ]);

        emojis.retain(|e| {
            reaction_map.contains_key(
                e.name
                    .to_lowercase()
                    .trim_end()
            )
        });

        for emoji in emojis {
            if msg.content
                .to_lowercase()
                .contains(reaction_map[&emoji.name.as_str().trim_end()]) {
                msg.react(ctx, emoji).await?;   
            }
        }
    };

    Ok(())
}