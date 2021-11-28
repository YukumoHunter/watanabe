use crate::database;

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

use database::profile::{set_profile, get_profile, UserProfile};

#[command]
async fn catchphrase(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut id: u64;

    // Get own catchphrase
    if args.is_empty() {
        id = msg.author.id
        .as_u64()
        .to_owned();

        let catchphrase: String;
        match get_profile(id) {
            Some(c) => catchphrase = c,
            None => catchphrase = "This person has no catchphrase, fail!".to_string()
        }

        msg.reply(ctx, format!("Your catchprase: {}", catchphrase)).await?;

        return Ok(());
    }

    // Set own catchphrase if the first argument is "set"
    let arg = args
        .single::<String>()
        .unwrap()
        .to_lowercase();
    let catchphrase = args.rest().to_string();
    if arg == "set" {
        let profile: UserProfile;
        if catchphrase.trim().is_empty() {
            profile = UserProfile::new(
                msg.author.id.as_u64().to_owned(),
                None
            );
        } else {
            profile = UserProfile::new(
                msg.author.id.as_u64().to_owned(),
                Some(catchphrase.to_owned())
            );
        }
        set_profile(profile).expect("Error setting profile");

        msg.reply(ctx, format!("Successfully set catchphrase to {}", catchphrase.as_str())).await?;

        return Ok(());
    }

    // Get catchphrases of other users
    let mut reply = String::new();
    for mention in &msg.mentions {
        id = mention.id
        .as_u64()
        .to_owned();
        
        let catchphrase: String;
        match get_profile(id) {
            Some(c) => catchphrase = c,
            None => catchphrase = "This person has no catchphrase, fail!".to_string()
        }
        
        reply += format!("{}'s catchphrase: {}\n", mention.name, catchphrase).as_str();
    };
    msg.reply(ctx, reply).await?;
    
    // User failed to construct a proper command
    if msg.mentions.is_empty() {
        msg.reply(ctx, "This user failed to properly use the command, laugh at this user!").await?;
    }

    Ok(())
}