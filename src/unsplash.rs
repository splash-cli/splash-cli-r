use serde::{Deserialize,Serialize};
use reqwest::{Error, Client};
use std::thread::Builder;
use std::io::copy;
use std::fs::File;

#[derive(Debug,Deserialize)]
pub(crate) struct PhotoUrls {
    pub full: String,
    pub regular: String,
    pub thumb: String,
    pub raw: String,
}

#[derive(Debug,Deserialize)]
pub(crate) struct Photo {
    pub id: String,
    pub color: String,
    pub width: u32,
    pub height: u32,
    pub urls: PhotoUrls
}

#[derive(Debug,Deserialize)]
pub(crate) struct LambdaResponse {
    pub id: String
}

pub(crate) struct Unsplash {
    pub base_url: String,
    pub client_id: String,
}

impl Unsplash {
    pub async fn get_random(&self) -> Result<Photo,Error> {
        self.get_photo("random").await
    }

    pub async fn get_photo_of_the_day(&self) -> Result<Photo, Error> {
        let response = reqwest::get("https://lambda.splash-cli.app/api").await?;
        let data: LambdaResponse = response.json().await?;

        self.get_photo(&data.id).await
    }

    pub async fn get_photo(&self, id: &str) -> Result<Photo, Error> {
        let client = Client::new();

        let response = client
            .get(&format!("{base}/photos/{id}", base=self.base_url,id=id))
            .header("Authorization", format!("Client-ID {key}", key=self.client_id))
            .send()
            .await?;

        match response.json().await {
            Ok(photo) => self.download(photo).await,
            Err(e) => Err(e)
        }
    }

    async fn download(&self, photo: Photo) -> Result<Photo, Error>{
        let mut file = File::create(format!("{filename}.txt", filename=photo.id))?;
        reqwest::get(&photo.urls.full).await?.copy_to(&mut file)?;

        Ok(photo)
    }
}
