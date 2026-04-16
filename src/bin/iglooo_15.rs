use std::collections::HashSet;

#[derive(Debug)]
enum Sentiment {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug)]
struct ParsedPost {
    text: String,
    hashtags: Vec<String>,
    mentions: Vec<String>,
    links: Vec<String>,
    sentiment: Sentiment,
}

fn extract_hashtags(post: &str) -> Vec<String> {
    let mut hashtags: Vec<String> = Vec::new();

    for word in post.split_whitespace() {
        if word.starts_with('#') {
            hashtags.push(word.to_string());
        }
    }
    hashtags
}

fn extract_mentions(post: &str) -> Vec<String> {
    let mut mentions: Vec<String> = Vec::new();

    for words in post.split_whitespace() {
        if words.starts_with("@") {
            mentions.push(words.to_string());
        }
    }
    mentions
}

fn extract_links(post: &str) -> Vec<String> {
    let mut links: Vec<String> = Vec::new();

    for words in post.split_whitespace() {
        if words.starts_with("http") || words.starts_with("https") || words.starts_with("www") {
            links.push(words.to_string());
        }
    }
    links
}

fn analyze_sentiment(content: &str) -> Sentiment {
    let positive_words: HashSet<&str> = [
        "good",
        "great",
        "awesome",
        "love",
        "fantastic",
        "excellent",
        "happy",
        "nice",
        "amazing",
    ]
    .iter()
    .cloned()
    .collect();

    let negative_words: HashSet<&str> = [
        "bad",
        "terrible",
        "awful",
        "hate",
        "worst",
        "sad",
        "angry",
        "horrible",
        "disgusting",
    ]
    .iter()
    .cloned()
    .collect();

    let mut positive_count = 0;
    let mut negative_count = 0;

    for word in content
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
    {
        if positive_words.contains(word) {
            positive_count += 1;
        } else if negative_words.contains(word) {
            negative_count += 1;
        }
    }

    match positive_count.cmp(&negative_count) {
        std::cmp::Ordering::Greater => Sentiment::Positive,
        std::cmp::Ordering::Less => Sentiment::Negative,
        _ => Sentiment::Neutral,
    }
}

fn parsed_post(text: &str) -> ParsedPost {
    let hashtags = extract_hashtags(text);
    let mentions = extract_mentions(text);
    let links = extract_links(text);
    let sentiment = analyze_sentiment(text);

    ParsedPost {
        text: text.to_string(),
        hashtags: hashtags,
        mentions: mentions,
        links: links,
        sentiment: sentiment,
    }
}

// todo : add regex for better pattern handling
// todo : unicode handling in hashtags and mentions
// todo : add function for finding trending topics

fn main() {
    let extracted_hashtags =
        extract_hashtags("@user said check out #coolstuff at www.example.com! Love it 😍 #awesome");

    let extracted_mentions =
        extract_mentions("@user said check out #coolstuff at www.example.com! Love it 😍 #awesome");

    let extracted_links =
        extract_links("@user said check out #coolstuff at www.example.com! Love it 😍 #awesome");

    let sentiment = analyze_sentiment(
        "@user said check out #coolstuff at www.example.com! Love it 😍 #awesome",
    );

    let parsedpost =
        parsed_post("@user said check out #coolstuff at www.example.com! Love it 😍 #awesome");

    println!("extracted_hashtags : {:?}", extracted_hashtags);
    println!("extracted_mentions : {:?}", extracted_mentions);
    println!("extracted_links : {:?}", extracted_links);
    println!("sentiment : {:?}", sentiment);
    println!("parsed post : {:?}", parsedpost);
}
