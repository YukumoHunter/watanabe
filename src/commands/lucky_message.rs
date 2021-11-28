use rand::Rng;
use serenity::{client::Context, framework::standard::CommandResult, model::{channel::Message, prelude::{Activity, OnlineStatus}}};

pub async fn lucky_message(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.author.bot { return Ok(()) }

    let num: u32 = rand::thread_rng().gen_range(0..1000);

    ctx.set_presence(
        Some(Activity::playing(format!("the game: {}", num))),
        OnlineStatus::Online
    ).await;
    
    if num == 0 {
        // The extra figo case
        if msg.author.id == 529978387356188672 && rand::thread_rng().gen_range(0..1000) == 0 {
            msg.reply(ctx, "ULTIMATE Breh moment").await?;
            return Ok(());
        }
        msg.reply(ctx, "Breh moment").await?;
    }
    
    Ok(())
}