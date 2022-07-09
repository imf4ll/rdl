use crate::utils::types::Format;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
struct GuestTokenResponse {
    guest_token: String,
}

#[derive(Debug, Deserialize)]
struct TempFormat {
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

pub fn get_video(url: String) -> Vec<Format> {
    let video_id: &str = url.split("/").collect::<Vec<&str>>()[5];

    let formats_req = Client::new();
    let formats_res = formats_req
        .get(format!("https://twitter.com/i/api/graphql/LJ_TjoWGgNTXCl7gfx4Njw/TweetDetail?variables=%7B%22focalTweetId%22%3A%22{video_id}%22%2C%22with_rux_injections%22%3Afalse%2C%22includePromotedContent%22%3Atrue%2C%22withCommunity%22%3Atrue%2C%22withQuickPromoteEligibilityTweetFields%22%3Atrue%2C%22withBirdwatchNotes%22%3Afalse%2C%22withSuperFollowsUserFields%22%3Atrue%2C%22withDownvotePerspective%22%3Afalse%2C%22withReactionsMetadata%22%3Afalse%2C%22withReactionsPerspective%22%3Afalse%2C%22withSuperFollowsTweetFields%22%3Atrue%2C%22withVoice%22%3Atrue%2C%22withV2Timeline%22%3Afalse%2C%22__fs_dont_mention_me_view_api_enabled%22%3Afalse%2C%22__fs_interactive_text_enabled%22%3Atrue%2C%22__fs_responsive_web_uc_gql_enabled%22%3Afalse%7D"))
        .header("Host", "twitter.com")
        .header("Authorization", "Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA")
        .header("X-Guest-Token", get_token())
        .send()
        .expect("Failed to request video information")
        .text()
        .expect("Failed to parse information");

    let formats: Vec<TempFormat> = serde_json::from_str(
        formats_res
            .split("\"variants\":").collect::<Vec<&str>>()[1]
            .split("}}]},\"favorite_count\"").collect::<Vec<&str>>()[0]
    ).unwrap();

    let mut qualities: Vec<Format> = vec![];

    for format in formats {
        let quality_vec: Vec<&str> = format.url.split("/").collect();
        let quality = quality_vec[7];

        if format.content_type == "video/mp4" {
            qualities.push(Format {
                quality: quality.to_string(),
                url: format.url,
            });
        }
    }

    qualities
}
