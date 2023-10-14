use structopt::StructOpt;
use teloxide::prelude::*;

fn main()
{
	println!("Enter main");
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
	println!("Enter async main");

	let bot = Bot::new(token);

	teloxide::repl(bot, |bot: Bot, msg: Message| async move {
		println!("Got message {:?}", msg);
		bot.send_dice(msg.chat.id).await?;
		Ok(())
	})
	.await;
}
