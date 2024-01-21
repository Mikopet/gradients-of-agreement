use serenity::builder::*;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::CreateQuickModal;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let modal = CreateQuickModal::new("What is your topic to reach consensus in?")
        .timeout(std::time::Duration::from_secs(600))
        .short_field("Title")
        .paragraph_field("Description");
    let response = interaction.quick_modal(ctx, modal).await?.unwrap();

    let inputs = response.inputs;
    let (title, description) = (&inputs[0], &inputs[1]);

    let embed = CreateEmbed::new()
        .title(title)
        .description(description)
        .image("attachment://goa.png")
        .timestamp(Timestamp::now());

    response
        .interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .embed(embed)
                    .add_file(CreateAttachment::path("./goa.png").await?),
            ),
        )
        .await?;
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("goa").description("creates a Gradients of Agreement poll")
}
