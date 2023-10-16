use structopt::StructOpt;
use teloxide::prelude::*;

fn main()
{
	let opt = Opt::from_args();

	tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.unwrap()
		.block_on(async_main(&opt.token));
}

#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "telegram_not", about = "Test telegram bot")]
struct Opt
{
	#[structopt(long)]
	token: String,
}

async fn async_main(token: &str)
{
	println!("Creating the bot...");

	let bot = Bot::new(token);

	println!("The bot is created...");

	teloxide::repl(bot, |bot: Bot, msg: Message| async move {
		let (kind, chat) = (msg.kind, msg.chat);
		match kind
		{
			teloxide::types::MessageKind::Common(teloxide::types::MessageCommon {
				from,
				media_kind: teloxide::types::MediaKind::Text(teloxide::types::MediaText { text, .. }),
				..
			}) =>
			{
				println!(
					"\n\n\n======The Bot======\nFrom user {} got message:\n\n\n{}",
					from.map(|u| u.id.to_string()).unwrap_or("unknown".to_string()),
					text
				);
			},
			_ =>
			{
				println!("\n\n\nGot unssupported message.");
			},
		}

		bot.send_message(
			chat.id,
			"Уважаемая администрация рассмотрит ваше сообщение в ближайшее время.",
		)
		.await?;
		Ok(())
	})
	.await;
}
