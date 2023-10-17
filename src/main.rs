use structopt::StructOpt;
use teloxide::{prelude as tel, requests::Requester, types as tel_t};

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

	let bot = tel::Bot::new(token);

	println!("Setting bot commands...");

	bot.set_my_commands([tel_t::BotCommand {
		command: START_COMMAND_NAME.to_string(),
		description: START_COMMAND_DESCRIPTION.to_string(),
	}])
	.await
	.unwrap();

	println!("The bot is created. Processing messages. Use Ctrl+C to stop the bot.");
	println!(
		"Input messages will be printed into stdout, use stdout redirection into a file in order to save input \
		 messages."
	);

	teloxide::repl(bot, handle_message).await;
}

async fn handle_message(bot: tel::Bot, msg: tel::Message) -> tel::ResponseResult<()>
{
	match &msg.kind
	{
		tel_t::MessageKind::Common(tel_t::MessageCommon {
			from,
			media_kind: tel_t::MediaKind::Text(tel_t::MediaText { text, .. }),
			..
		}) =>
		{
			if text.as_str().trim_start().starts_with(START_COMMAND_NAME)
			{
				log_print_header();
				println!(
					"From user {} got start command.",
					from.as_ref().map(|u| u.id.to_string()).unwrap_or("unknown".to_string())
				);

				if let Err(e) = bot.send_message(msg.chat.id, START_COMMAND_RESPONSE).await
				{
					log_print_header();
					println!("\n\n\nFailed to responde: {}.\n\n", e);
				}
			}
			else
			{
				log_print_header();
				println!(
					"From user {} got message:\n\n\n{}",
					from.as_ref().map(|u| u.id.to_string()).unwrap_or("unknown".to_string()),
					text
				);

				if let Err(e) = bot.send_message(msg.chat.id, DEFAULT_BOT_RESPONSE).await
				{
					log_print_header();
					println!("\n\n\nFailed to responde: {}.\n\n", e);
				}
			}
		},
		_ =>
		{
			log_print_header();
			println!("Got unssupported message.\n");
		},
	}

	Ok(())
}

fn log_print_header()
{
	println!("\n\n======The Bot======");
}

const START_COMMAND_NAME: &str = "/start";
const START_COMMAND_DESCRIPTION: &str = "Начать общение";
const START_COMMAND_RESPONSE: &str =
	"Отправь свою историю или откровение для публикации в @overhear_it.\nВсе публикации анонимны. Удачи!";
const DEFAULT_BOT_RESPONSE: &str = "Уважаемая администрация рассмотрит ваше сообщение в ближайшее время.\nА пока вы \
                                    можете подписаться на @overhear_it.";
