use structopt::StructOpt;
mod unsplash;
use tokio::prelude::*;
use crate::unsplash::Photo;
use serde::export::fmt::Display;
use core::fmt;


#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Cli {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(long)]
    day: bool,

    #[structopt(long,required=false,default_value="")]
    id: String,
}

fn no_photo() {
    println!("No photo!")
}

#[tokio::main]
async fn main() {
    let cli = Cli::from_args();

    let api = unsplash::Unsplash {
        base_url: "https://api.unsplash.com".to_string(),
        client_id: "e16e0a00f80aa7f1d491201d5db32bfdfd801d9be57b05a2b959436432e55d71".to_string()
    };

    if cli.day {
        let photo = to_option(api.get_photo_of_the_day().await);

        match photo {
            Some(p) => {
                println!("{id} {color}", color=p.color,id=p.id)
            },
            None => {
                no_photo()
            }
        }
        return
    }

    if cli.id != "" {
        let photo = to_option(api.get_photo(&cli.id).await);

        match photo {
            Some(p) => {
                println!("{id} {color}", color=p.color,id=p.id)
            },
            None => {
                no_photo()
            }
        }
    }
}


fn to_option<T, U>(result: Result<T, U>) -> Option<T> {
    match result {
        Ok(data) => Option::from(data),
        Err(e) => None
    }
}
