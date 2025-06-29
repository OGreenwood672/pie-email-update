use mail_send::SmtpClientBuilder;
use mail_builder::MessageBuilder;

pub struct Message {
    pub from_email: String,
    pub from_password: String,
    pub to: String,
    pub subject: String,
    pub html_body: String,
}

pub fn send_msg_blocking(msg: &Message) -> Result<(), Box<dyn std::error::Error>> {
    // Use a simple async runtime to block on the async function
    tokio::runtime::Runtime::new()?.block_on(send_msg(msg))
}

async fn send_msg(msg: &Message) -> Result<(), Box<dyn std::error::Error>> {
    let message = MessageBuilder::new()
        .from(msg.from_email.clone())
        .to(msg.to.clone())
        .subject(&msg.subject)
        .html_body(msg.html_body.clone());

    let mut smtp = SmtpClientBuilder::new("smtp.gmail.com", 587)
        .implicit_tls(false)
        .credentials((&*msg.from_email, &*msg.from_password))
        .connect()
        .await?; // Connect to the SMTP server

    smtp.send(message).await?; // Send the message

    Ok(())
}