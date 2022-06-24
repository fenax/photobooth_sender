extern crate gphoto;
extern crate telegram_bot;
extern crate chrono;
extern crate futures;
extern crate tokio;


use std::path::Path;
use std::env;
use futures::executor::block_on;
use futures::StreamExt;
use telegram_bot::*;

use chrono::{DateTime, Utc};


#[tokio::main]
async fn main() -> Result<(), Error>  {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let filename = env::var("FILENAME").expect("FILENAME not set");

    let api = Api::new(token);
    let mut stream = api.stream();
    let channel = ChatRef::ChannelUsername("@Fluufff_Photobooth".to_string());


/*    println!("Hello, world!");
    let mut context = gphoto::Context::new().unwrap();

    let mut camera = gphoto::Camera::autodetect(&mut context).unwrap();
    let capture = camera.capture_image(&mut context).unwrap();

    let now: DateTime<Utc> = Utc::now();
    let filename: String;
    filename = now.format("%Y-%m-%d--%H-%M-%S.jpg").to_string();
    let path = Path::new(&filename);

    let mut file = gphoto::FileMedia::create(&path).unwrap();

    camera.download(&mut context, &capture, &mut file).unwrap();
*/        
    let file = InputFileUpload::with_path(filename);
    api.send(channel.photo(&file)).await?;
    Ok((()))
}
