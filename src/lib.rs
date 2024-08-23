#[doc = include_str!("../README.md")]
use anyhow::Result;
use bytes::Buf;
use reqwest::Url;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

//--------------------------------------------------------------------------------------------------

/**
Wrapper around a blocking [`reqwest`] [`Client`] providing convenience methods
*/
pub struct Client {
    client: reqwest::blocking::Client,
}

impl Client {
    /**
    Create a new [`Client`] with the given user agent
    */
    pub fn new(user_agent: &str) -> Result<Client> {
        Ok(Client {
            client: reqwest::blocking::Client::builder()
                .user_agent(user_agent)
                .build()?,
        })
    }

    /**
    Download a URL and return the contents in a string
    */
    pub fn to_string(&self, url: &str) -> Result<String> {
        Ok(self.client.get(url).send()?.text()?)
    }

    /**
    Download a URL to a file in the current or optional destination directory
    */
    pub fn to_file(&self, url: &str, dst: Option<&Path>) -> Result<PathBuf> {
        let res = self.client.get(url).send()?;

        let path = dst
            .and_then(|x| {
                if x.is_dir() {
                    Some(x.join(url_filename(res.url())))
                } else {
                    None
                }
            })
            .unwrap_or_else(|| PathBuf::from(url_filename(res.url())));

        std::io::copy(&mut res.bytes()?.reader(), &mut File::create(&path)?)?;

        Ok(path)
    }
}

//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    use reqwest::Url;

    #[test]
    fn test_url_filename() {
        println!();
        for (i, (url, filename)) in [
            ("http://some.host.tld/path/to/file.ext", "file.ext"),
            ("http://some.host.tld/path/to/", "to"),
            ("http://some.host.tld/path/to", "to"),
            ("http://some.host.tld/path/", "path"),
            ("http://some.host.tld/path", "path"),
            ("http://some.host.tld/", "some.host.tld.html"),
            ("http://some.host.tld", "some.host.tld.html"),
        ]
        .iter()
        .enumerate()
        {
            println!("{}. {url:?} => {filename:?}", i + 1);
            assert_eq!(url_filename(&Url::parse(url).unwrap()), *filename);
        }
    }
}

/**
Get the filename from a [`Url`]

* Last non-empty path segment
* If there are none, return the host with `.html` appended
*/
fn url_filename(url: &Url) -> String {
    url.path_segments()
        .and_then(|x| {
            for s in x.rev() {
                if !s.is_empty() {
                    return Some(s.to_string());
                }
            }
            None
        })
        .unwrap_or_else(|| format!("{}.html", url.host_str().unwrap()))
}
