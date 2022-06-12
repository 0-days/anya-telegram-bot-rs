use std::{error::Error, collections::HashSet};
use std::borrow::Cow;

use regex::Regex;
use lazy_static::lazy_static;
use reqwest;
use reqwest::header::REFERER;
use serde::{ Deserialize };

use teloxide::{ prelude::*, types::ParseMode };
use teloxide::{ utils::markdown::{ link, bold, escape } };

#[derive(Deserialize, Debug)]
struct MetaData {
    // #[serde(flatten)]
    items: Vec<Item>,
}

#[derive(Deserialize, Debug)]
struct Item {
    // id: String,
    snippet: Snippet,
    statistics: Statistics,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Snippet {
    title: String,
    published_at: String,
    channel_id: String,
    thumbnails: Thumbnails,
    channel_title: String,
    default_language: Option<String>,
    default_audio_language: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Thumbnails {
    default: Option<Thumbnail>,
    medium: Option<Thumbnail>,
    high: Option<Thumbnail>,
    standard: Option<Thumbnail>,
    maxres: Option<Thumbnail>,
}

#[derive(Deserialize, Debug)]
struct Thumbnail {
    url: String,
    // width: u32,
    // height: u32,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Statistics {
    view_count: String,
    like_count: String,
    comment_count: String,
}

impl MetaData {
    fn fmt(&self) -> String {
        let snippet = &self.items[0].snippet;
        let stat = &self.items[0].statistics;
        let mut s = format!(
            // "kind: {}\n\
            // etag: {}", self.kind, self.etag
            // "{:#?}", self
            "メタデータはんめいしたー\n\n\
            {}\n\
            公開日: {}\n\
            閲覧回数: {}\n\
            ライク数: {}\n\
            コメント数: {}\n",
            bold(&escape(&snippet.title)),
            &escape(&snippet.published_at),
            bold(&stat.view_count),
            bold(&stat.like_count),
            bold(&stat.comment_count),
        );

        if let Some(lang) = &snippet.default_language {
            s.push_str(&format!("言語: {}\n", escape(&lang)))
        }
        if let Some(lang) = &snippet.default_audio_language {
            s.push_str(&format!("音声言語: {}\n", escape(&lang)))
        }
        s.push_str(&format!("{} より\n\n", link(&format!("https://youtube.com/channel/{}", &snippet.channel_id), &escape(&snippet.channel_title))));

        let thumbnails = &snippet.thumbnails;
        s.push_str("サムネイル:\n");
        if let Some(default) = &thumbnails.default {
            s.push_str(&format!("▫️{}\n", link(&default.url, "デフォルト")))
        }
        if let Some(medium) = &thumbnails.medium {
            s.push_str(&format!("▫️{}\n", link(&medium.url, "低画質")))
        }
        if let Some(high) = &thumbnails.high {
            s.push_str(&format!("▫️{}\n", link(&high.url, "高画質")))
        }
        if let Some(standard) = &thumbnails.standard {
            s.push_str(&format!("▫️{}\n", link(&standard.url, "スタンダート")))
        }
        if let Some(maxres) = &thumbnails.maxres {
            s.push_str(&format!("▫️{}\n", link(&maxres.url, "最高画質")))
        };
        s
    }
}

pub fn contains_yt(m: Message) -> bool {
    const P: &str = "youtube.com/";
    return m.text().unwrap().contains(P)
}

fn extract_query(s: &str) -> HashSet<Cow<str>> {
    lazy_static! {
        static ref R: Regex = Regex::new(r"((?:https?:)?//)?((?:www|m)\.)?((?:youtube(-nocookie)?\.com|youtu.be))(/(?:[\w\-]+\?v=|embed/|v/)?)(?P<query>([\w\-]+)(\S+)?)")
        .unwrap();
    }

    let q: HashSet<_> = R
        .captures_iter(s)
        .map(|c| match c.name("query") {
            Some(v) => Cow::from(v.as_str()),
            // None => ,
            _ => unreachable!(),
        })
        .collect();

    q
}

async fn metadata(s: &str) -> Result<MetaData, reqwest::Error> {
    const DEST: &str = "https://www.googleapis.com/youtube/v3/videos?key=AIzaSyAa-o55aIMt4YC0mhPyp8WfGql5DVg_fp4&part=snippet,statistics,recordingDetails,status,liveStreamingDetails,localizations,contentDetails,topicDetails&id=";
    const TAIL: &str = "&_=1654902130800";

    let client = reqwest::Client::new();
    let body = client
        .get(format!("{DEST}{s}{TAIL}"))
        .header(REFERER, "https://mattw.io/")
        .send()
        .await?
        .json::<MetaData>()
        .await?;
    Ok(body)
}

pub async fn answer(b: AutoSend<Bot>, m: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    let qs = extract_query(m.text().unwrap());
    for q in qs {
        let meta = metadata(q.as_ref())
            .await?;
            // .unwrap();
        b.send_message(m.chat.id, &meta.fmt())
            .reply_to_message_id(m.id)
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
    }
    Ok(())
}