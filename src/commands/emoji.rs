use serenity::client::Context;
use serenity::model::{
    channel::Message,
};
use serenity::framework::standard::{
    Args,
    CommandResult,
    macros::{
        command,
    }
};

#[command]
async fn emoji(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if msg.is_private() { return Ok(()) };
    
    let arg: String = args.single().expect("Could not parse arguments");
    let id = msg.guild_id.expect("Error retrieving guild ID");
    let emojis = id.emojis(&ctx.http).await?;
    
    if let Some(emoji) = emojis.iter().find(|e| e.name == arg) {
        if emoji.animated {
            msg.reply(ctx, format!("<a:{}:{}>", emoji.name, emoji.id)).await?;
        } else {
            msg.reply(ctx, format!("<:{}:{}>", emoji.name, emoji.id)).await?;
        }
    } else {
        msg.reply_mention(ctx, "failed to correctly use the emoji command, laugh at this user").await?;
    }

    Ok(())
}