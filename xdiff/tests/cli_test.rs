use std::str::FromStr;

use reqwest::{Method, header::HeaderMap, StatusCode};
use serde_json::json;
use url::Url;
use xdiff::{cli::{KeyVal, KeyValType}, ExtraArgs, RequestProfile, ResponseExt, ResponseProfile, ValidateConfig, get_status_text, get_header_text};

#[test]
fn cli_from_vec_key_val_for_extra_args() {
    let args = vec![
        KeyVal {
            key_type: KeyValType::Header,
            key: "header_key".into(),
            value: "header_val".into(),
        },
        KeyVal {
            key_type: KeyValType::Query,
            key: "query_key".into(),
            value: "query_val".into(),
        },
        KeyVal {
            key_type: KeyValType::Body,
            key: "body_key".into(),
            value: "body_val".into(),
        },
    ];
    let extra_args = ExtraArgs::from(args);
    assert_eq!(extra_args, ExtraArgs {
        headers: vec![("header_key".into(), "header_val".into())],
        query: vec![("query_key".into(), "query_val".into())],
        body: vec![("body_key".into(), "body_val".into())],
    })
}


#[tokio::test]
async fn response_ext_get_text_should_work() {
    // mock server
    let mut server = mockito::Server::new();
    let _mock = server.mock("GET", "/todo")
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body(serde_json::to_string(&json!({"id": 1, "title": "todo"})).unwrap())
    .create();
    let url = format!("{}{}", server.url(), "/todo");
    let res = get_response(&url, &ExtraArgs::default()).await;
    
    let response_profile = ResponseProfile::new(
      vec!["connection".into(), "content-length".into(), "date".into()],
      vec!["id".into()],
    );
    let res = res.get_text(&response_profile).await.unwrap();
    assert_eq!(
        res,
        "HTTP/1.1 200 OK\ncontent-type: \"application/json\"\n\n{\n  \"title\": \"todo\"\n}"
    );
}

#[tokio::test]
async fn response_ext_get_header_keys_should_work() {
    let mut server = mockito::Server::new();
    let _mock = server.mock("GET", "/todo")
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body(serde_json::to_string(&json!({"id": 1, "title": "todo"})).unwrap())
    .create();
    let url = format!("{}{}", server.url(), "/todo");
    let res = get_response(&url, &Default::default()).await;

    let mut sorted_header_keys = res.get_header_keys();
    sorted_header_keys.sort();
    let mut expected_header_keys = vec!["connection", "content-length", "content-type", "date"];
    expected_header_keys.sort();
    assert_eq!(sorted_header_keys, expected_header_keys);
}

#[test]
fn request_profile_validate_should_work() {
    let server = mockito::Server::new();
    let url = format!("{}{}", server.url(), "/todo?a=1&b=2");
    let profile: RequestProfile = get_profile(&url);
    assert!(profile.validate().is_ok());
}

#[test]
fn request_profile_with_bad_param_validate_should_fail() {
    let profile = RequestProfile::new(
        Method::GET,
        Url::parse("http://localhost:8080/todo").unwrap(),
        Some(json!([1, 2, 3])),
        HeaderMap::new(),
        None,
    );
    let res = profile.validate();
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "Params must be an object but got\n- 1\n- 2\n- 3\n"
    );
}

#[tokio::test]
async fn get_status_text_should_work() {
    let mut server = mockito::Server::new();
    let _mock = server.mock("GET", "/todo")
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body(serde_json::to_string(&json!({"id": 1, "title": "todo"})).unwrap())
    .create();
    let url = format!("{}{}", server.url(), "/todo");
    let res = get_response(&url, &Default::default()).await.into_inner();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(get_status_text(&res).unwrap(), "HTTP/1.1 200 OK\n");
}

#[tokio::test]
async fn get_headers_text_should_work() {
    let mut server = mockito::Server::new();
    let _mock = server.mock("GET", "/todo")
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body(serde_json::to_string(&json!({"id": 1, "title": "todo"})).unwrap())
    .create();
    let url = format!("{}{}", server.url(), "/todo");
    let res = get_response(&url, &Default::default()).await.into_inner();
    assert_eq!(
        get_header_text(&res, &["content-length".into(), "connection".into(), "date".into()]).unwrap(),
        "content-type: \"application/json\"\n\n"
    )
}

async fn get_response(path_and_query: &str, args: &ExtraArgs) -> ResponseExt {
    let profile = get_profile(path_and_query);
    profile.send(args).await.unwrap()
}

fn get_profile(url: &str) -> RequestProfile {
    RequestProfile::from_str(&url).unwrap()
}
