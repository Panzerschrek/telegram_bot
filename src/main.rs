use teloxide::prelude::*;

fn main()
{
	println!("Enter main");
	tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.unwrap()
		.block_on(async_main());
}

async fn async_main()
{
	println!("Enter async main");

	let bot = Bot::new("token here");

	teloxide::repl(bot, |bot: Bot, msg: Message| async move {
		println!("Got message {:?}", msg);
		bot.send_dice(msg.chat.id).await?;
		Ok(())
	})
	.await;
}
