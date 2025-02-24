// Copyright 2021 Rafael Mardojai CM
// SPDX-License-Identifier: GPL-3.0-or-later

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::collections::HashMap;
use std::error;

use url::Url;
use scraper::{Html, Selector};
use scraper::element_ref::ElementRef;

use super::{Data, Image, CLIENT};

pub async fn scrape(url: &Url) -> Result<Data, Error> {
    //! Request URL html body and scrape it to get the needed data

    let mut resp = CLIENT.get(&url).await?;

    if resp.status().is_success() {
        let mut metadata = HashMap::new(); // Empty HashMap<String, String> to store meta tags
        let mut images = Vec::new(); // Empty Vec<Image> to store images from HTML

        // Call function to get data from html:
        get_html_data(&resp.body_string().await?, &mut metadata, &mut images).await; // Write html data to a Vec<>

        let mut data = Data::default();
        data.url = url.host_str().unwrap().to_string(); // Set Data URL
        data.metadata = metadata;
        // Collect a new Images Vec<> with the relative URLs normalized:
        data.images = images.iter().map(|i| {
            let mut i = i.clone();
            i.normalize(&url);
            i
        }).collect::<Vec<Image>>();

        Ok(data)
    } else {
        Err(Error::Unexpected)
    }
}

async fn get_html_data(text: &String,
                 meta: &mut HashMap<String, String>,
                 images: &mut Vec<Image>) {
    //! Parse html and get data

    let document = Html::parse_document(&text); // HTML document from request text

    // Get document title
    let selector = Selector::parse("title").unwrap(); // HTML <title> selector
    let title = document.select(&selector).next().unwrap();
    // Push title to the meta Vec<>
    meta.insert(String::from("title"), title.inner_html().trim().to_string());

    // Get meta tags
    let selector = Selector::parse("meta").unwrap();
    for element in document.select(&selector) {
        match get_meta_prop(&element, "property") {
            Some((key, content)) => {
                let mut content = content.trim().to_string();
                content = content.replace('\n', " ");
                meta.insert(key, content);
            }
            None => (),
        }
        match get_meta_prop(&element, "name") {
            Some((key, content)) => {
                let mut content = content.trim().to_string();
                content = content.replace('\n', " ");
                meta.insert(key, content);
            },
            None => (),
        }
    }

    // Get images
    let selector = Selector::parse("img").unwrap();
    for element in document.select(&selector) {
        match element.value().attr("src") {
            Some(src) => {
                let src = src.trim().to_string();
                if src.contains(".jpg") || src.contains(".jpeg") || src.contains(".png"){
                    images.push(Image::new(src))
                }
            },
            None => (),
        }
    }
}

fn get_meta_prop(element: &ElementRef, name: &str) -> Option<(String, String)> {
    element.value().attr(name).and_then(|key|
        element.value().attr("content").map(|content| (key.to_string(), content.to_string())))
}

#[derive(Debug)]
pub enum Error {
    NetworkError(surf::Error),
    Unexpected,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Error::NetworkError(ref e) => write!(f, "NetworkError:  {}", e),
            Error::Unexpected => write!(f, "UnexpectedError"),
        }
    }
}

impl From<surf::Error> for Error {
    fn from(err: surf::Error) -> Error {
        Error::NetworkError(err)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str { "" }
}
