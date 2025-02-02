use base64::{engine::general_purpose, Engine as _};
use discord_flows::{model::Message, Bot, ProvidedBot};
use flowsnet_platform_sdk::logger;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let discord_token = std::env::var("discord_token").unwrap();
    let bot = ProvidedBot::new(discord_token);
    bot.listen(|msg: Message| handler(&bot, msg)).await;
    Ok(())
}

async fn handler(bot: &ProvidedBot, msg: Message) {
    logger::init();
    let discord = bot.get_client();
    let binding = msg.content.trim();
    let message: Vec<&str> = binding.split(' ').collect();
    let mut resp;

    if msg.author.bot {
        log::debug!("ignored bot message");
        return;
    }
    if msg.member.is_some() {
        log::debug!("ignored channel message");
        return;
    }

    match message[0].to_lowercase().as_str() {
        "" => resp = format!("Empty input I shall do nothing"),
        "encode" => {
            let string = message[1..].join(" ");
            let encoded = general_purpose::STANDARD.encode(&string);
            resp = format!(
                "Your original string was {string} \n Your base64 encoded string is {encoded} \n"
            )
        }
        "decode" => {
            let string = message[1].trim().to_owned();
            let decoded = general_purpose::STANDARD.decode(string.clone());

            match decoded{
                Ok(decoded_vec) =>{
                    match String::from_utf8(decoded_vec){
                    Ok(decoded_str)=> {resp = format!(
                    "Your original string was {string} \n Your base64 decoded string is {decoded_str} \n"
                )}
                Err(_) => resp = format!("Something went wrong while converting to string"),
            
            }}
                Err(_) => resp = format!("Something went wrong while while decoding string"),
                 }

            
        }
        _ => {
            resp = format!("Invalid method only encode and decode are accepted methods the case does not matter");
        }
    }

    if message.len() < 2 {
        resp = format!("Please format your request in the form <Method> <Text>")
    }

    let channel_id = msg.channel_id;

    _ = discord
        .send_message(
            channel_id.into(),
            &serde_json::json!({
                "content": resp
            }),
        )
        .await;
}
