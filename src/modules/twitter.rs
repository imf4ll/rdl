use crate::utils::types::Format;
use reqwest::blocking::Client;
use serde::Deserialize;
use crate::logger;
use serde_json;

#[derive(Deserialize)]
struct GuestTokenResponse {
    guest_token: String,
}

#[derive(Debug, Clone, Deserialize)]
struct TweetResults {
    result: Result
}

#[derive(Debug, Clone, Deserialize)]
struct Result {
    rest_id : String,
    legacy: Legacy,
}

#[derive(Debug, Clone, Deserialize)]
struct Legacy {
    extended_entities: ExtendedEntities,
    full_text: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ExtendedEntities {
    media: Vec<Media>
}

#[derive(Debug, Clone, Deserialize)]
struct Media {
    video_info: VideoInfo,
}

#[derive(Debug, Clone, Deserialize)]
struct VideoInfo {
    variants: Vec<Variant>,
}

#[derive(Debug, Clone, Deserialize)]
struct Variant {
    content_type: String,
    url: String,
}

fn get_token() -> String {
    let guest_token_req = Client::new();

    let guest_token: GuestTokenResponse = guest_token_req
        .post("https://api.twitter.com/1.1/guest/activate.json")
        .header("Authorization", "Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA")
        .send()
        .expect("Failed to request guest token")
        .json()
        .expect("Failed to parse JSON body");

    guest_token.guest_token
}

pub fn get_video(url: String) -> (Vec<Format>, String) {
    let video_id: &str = url.split("/").collect::<Vec<&str>>()[5];

    let tweet_req = Client::new();
    let tweet_res = tweet_req
        .get(format!("https://twitter.com/i/api/graphql/LJ_TjoWGgNTXCl7gfx4Njw/TweetDetail?variables=%7B%22focalTweetId%22%3A%22{video_id}%22%2C%22with_rux_injections%22%3Afalse%2C%22includePromotedContent%22%3Atrue%2C%22withCommunity%22%3Atrue%2C%22withQuickPromoteEligibilityTweetFields%22%3Atrue%2C%22withBirdwatchNotes%22%3Afalse%2C%22withSuperFollowsUserFields%22%3Atrue%2C%22withDownvotePerspective%22%3Afalse%2C%22withReactionsMetadata%22%3Afalse%2C%22withReactionsPerspective%22%3Afalse%2C%22withSuperFollowsTweetFields%22%3Atrue%2C%22withVoice%22%3Atrue%2C%22withV2Timeline%22%3Afalse%2C%22__fs_dont_mention_me_view_api_enabled%22%3Afalse%2C%22__fs_interactive_text_enabled%22%3Atrue%2C%22__fs_responsive_web_uc_gql_enabled%22%3Afalse%7D"))
        .header("Host", "twitter.com")
        .header("Authorization", "Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA")
        .header("X-Guest-Token", get_token())
        .send()
        .expect("Failed to request video information")
        .text()
        .expect("Failed to parse information");

    if !tweet_res.contains("\"variants\":") {
        logger::error("Invalid video parsed.");

    }

    let mut all_medias: Vec<TweetResults> = vec![];

    for i in 0..tweet_res.matches("\"tweet_results\":{").count() + 1 {
        let json = &tweet_res
            .split("\"tweet_results\":").collect::<Vec<&str>>()[i]
            .split(",\"tweetDisplayType").collect::<Vec<&str>>()[0];

        if !json.contains("\"extended_entities\":{") {
            continue

        }

        let tweet: TweetResults = match serde_json::from_str::<TweetResults>(json) {
            Ok(m) => m,
            Err(_) => TweetResults {
                result: Result {
                    rest_id: String::from(""),
                    legacy: Legacy {
                        extended_entities: ExtendedEntities {
                            media: vec![] },
                            full_text: String::from("")
                    }
                }
            }
        };

        all_medias.push(tweet);
    }

    let mut media: TweetResults = all_medias
        .clone()
        .first()
        .expect("Unable to locate video information.")
        .to_owned();

    for m in all_medias {
        if m.result.rest_id.contains(url.split("/").collect::<Vec<&str>>().last().unwrap()) {
            media = m;

        }
    }

    let mut qualities: Vec<Format> = vec![];

    for format in &media.result.legacy.extended_entities.media[0].video_info.variants {
        let quality_vec: Vec<&str> = format.url.split("/").collect();
        let quality = quality_vec[quality_vec.len() - 2];

        if format.content_type == "video/mp4" {
            qualities.push(Format {
                quality: quality.to_string(),
                url: format.url.to_string(),
                audio: String::from(""),
            });
        }
    }

    return (qualities, media.result.legacy.full_text);
}
