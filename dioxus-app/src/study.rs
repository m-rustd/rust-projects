/*
 * @Author: misterzhou
 * @Date: 2023-11-24 11:56:52
 * @LastEditTime: 2023-11-24 11:56:54
 * @LastEditors: misterzhou
 * @FilePath: /dioxus-app/src/lib copy.rs
 * @Description: 
 */
use dioxus::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use futures::future::join_all;

pub static BASE_API_URL: &str = "https://hacker-news.firebaseio.com/v0/";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct StoryPageData {
    #[serde(flatten)]
    pub item: StoryItem,
    #[serde(default)]
    pub comments: Vec<Comment>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
struct StoryItem {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub text: Option<String>,
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub score: i64,
    #[serde(default)]
    pub descendants: i64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    pub r#type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
struct Comment {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub text: Option<String>,
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub score: i64,
    #[serde(default)]
    pub descendants: i64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    pub r#type: String,
}

// New
#[derive(Clone, Debug)]
enum PreviewState {
    Unset,
    Loading,
    Loaded(StoryPageData),
}

async fn get_story_preview(id: i64) -> Result<StoryItem, reqwest::Error> {
    let url = format!("{}item/{}.json", BASE_API_URL, id);
    reqwest::get(&url)
       .await?
       .json()
       .await
}

async fn get_stories(count: usize) -> Result<Vec<StoryItem>, reqwest::Error> {
    let url = format!("{}topstories.json", BASE_API_URL);
    let stories_ids = &reqwest::get(&url)
        .await?
        .json::<Vec<i64>>()
        .await?[..count];

    let story_futures = stories_ids[..usize::min(stories_ids.len(), count)]
        .iter()
        .map(|&story_id| get_story_preview(story_id));

    let stories = join_all(story_futures)
        .await
        .into_iter()
        .filter_map(|story| story.ok())
        .collect();
    Ok(stories)
}

async fn get_story(id: i64) -> Result<StoryPageData, reqwest::Error> {
    let url = format!("{}item/{}.json", BASE_API_URL, id);
    let mut story = reqwest::get(&url)
       .await?
       .json::<StoryPageData>()
       .await?;
    
      let comment_futures = story.item.kids.iter().map(|&id| get_comment(id));
      let comments = join_all(comment_futures)
          .await
          .into_iter()
          .filter_map(|c| c.ok())
          .collect();
  
      story.comments = comments;

      Ok(story)
}

// #[async_recursion::async_recursion(?Send)]
async fn get_comment_with_depth(id: i64, depth: i64) -> Result<Comment, reqwest::Error> {
    let url = format!("{}item/{}.json", BASE_API_URL, id);
    let comment = reqwest::get(&url).await?.json::<Comment>().await?;
    // if depth > 0 {
        // let sub_comments_futures = comment
        //     .kids
        //     .iter()
        //     .map(|story_id| get_comment_with_depth(*story_id, depth - 1));
        // comment.sub_comments = join_all(sub_comments_futures)
        //     .await
        //     .into_iter()
        //     .filter_map(|c| c.ok())
        //     .collect();
    // }
    Ok(comment)
}

async fn get_comment(comment_id: i64) -> Result<Comment, reqwest::Error> {
    let comment = get_comment_with_depth(comment_id, 2).await?;
    Ok(comment)
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || PreviewState::Unset);

    render! {
        div {
          display: "flex",
          flex_direction: "row",
          width: "100%",
          div {
              width: "50%",
              stories_component {}
          }
          div {
              width: "50%",
              preview_component {}
          }
      }
    }
}

async fn resolve_story(
    full_story: UseRef<Option<StoryPageData>>,
    preview_state: UseSharedState<PreviewState>,
    story_id: i64,
) {
    // 点击过直接取出
    if let Some(story) = &*full_story.read() {
        info!("Story already loaded: {story:?}");
        *preview_state.write() = PreviewState::Loaded(story.clone());
        return;
    }
    *preview_state.write() = PreviewState::Loading;
    if let Ok(story) = get_story(story_id).await {
        *preview_state.write() = PreviewState::Loaded(story.clone());
        *full_story.write() = Some(story);
    }
}

// New
fn stories_component(cx: Scope) -> Element {
    let stories: &UseFuture<Result<Vec<StoryItem>, reqwest::Error>> = use_future(cx, (), |_e|get_stories(10));

    match stories.value() {
        Some(Ok(list)) => {
            render! {
                div {
                    for story in list {
                        story_listing {
                            story: story.clone()
                        }
                    }
                }              
            }
        },
        Some(Err(e)) => {
            warn!("An error occurred while fetching stories {e}");
            render! {
                "An error occurred while fetching stories"
            }
        },
        None => {
            info!("loading stories");
            render! {
                "Loading stories..."
            }
        }
    }
}

fn preview_component(cx: Scope) -> Element {
    let preview_state = use_shared_state::<PreviewState>(cx)?;
    match &*preview_state.read() {
      PreviewState::Unset => render! {
          "Hover over a story to preview it here"
      },
      PreviewState::Loading => render! {
          "Loading..."
      },
      PreviewState::Loaded(story) => {
          let title = &story.item.title;
          let url = story.item.url.as_deref().unwrap_or_default();
          let text = story.item.text.as_deref().unwrap_or_default();
          render! {
              div {
                  padding: "0.5rem",
                  div {
                      font_size: "1.5rem",
                      a {
                          href: "{url}",
                          "{title}"
                      }
                  }
                  div {
                      dangerous_inner_html: "{text}",
                  }
                  for comment in &story.comments {
                      comment_component { comment: comment.clone() }
                  }
              }
          }
      }
  }
}

#[inline_props]
fn comment_component(cx: Scope, comment: Comment) -> Element<'a> {
    let text = comment.text.as_deref().unwrap_or_default();
    render! {
        div {
            padding: "0.5rem",
            div {
                color: "gray",
                "by {comment.by}"
            }
            div {
                dangerous_inner_html: "{text}"
            }
            // for kid in &comment.sub_comments {
            //     comment_component { comment: kid.clone() }
            // }
        }
    }
}

#[inline_props]
fn story_listing(cx: Scope, story: StoryItem) -> Element {
    let preview_state = use_shared_state::<PreviewState>(cx)?;
    let StoryItem {
        title,
        url,
        by,
        score,
        time,
        kids,
        id,
        ..
    } = story;
    let full_story = use_ref(cx, || None);

    let url = url.as_deref().unwrap_or_default();
    let hostname = url.trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");
    let score = format!("{score} {}", if *score == 1 { "point" } else { "points" });
    let comments = format!("{} {}", kids.len(), if kids.len() == 1 { "comment" } else { "comments" });
    let time = time.format("%Y-%m-%d %H:%M:%S");
    
    let caption_container_style = r#"
        font_size: "1.25rem",
        color: "gray",
    "#;

    render! {
        div {
          padding: "0.5rem",
          position: "relative",
          onmouseenter: move |_| {
              info!("div onMouseEnter");
              // *preview_state.write() = PreviewState::Loaded(StoryPageData {
              //     item: story.clone(),
              //     comments: vec![],
              // });
              resolve_story(full_story.clone(), preview_state.clone(), *id)
          },
          div {
              font_size: "1.5rem",
              a {
                  href: url,
                  onfocus: move |e| {
                      info!("focus {e:?}");
                      *preview_state.write() = PreviewState::Loaded(StoryPageData {
                          item: story.clone(),
                          comments: vec![],
                      });
                  },
                  "{title}"
              }
              a {
                  color: "gray",
                  href: "https://news.ycombinator.com/from?site={hostname}",
                  text_decoration: "none",
                  " {hostname}"
              }
          }
          div {
              class: "cl",
              style: "{caption_container_style}",
              div {
                  "{score}"
              }
              div {
                  padding_left: "0.5rem",
                  "by {by}"
              }
              div {
                  padding_left: "0.5rem",
                  "{time}"
              }
              div {
                  padding_left: "0.5rem",
                  "{comments}"
              }
          }
        }
    }
}
