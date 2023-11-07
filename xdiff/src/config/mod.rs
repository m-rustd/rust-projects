mod diff;
mod req;

use std::{str::FromStr, collections::HashSet};
use std::fmt::Write as _;

use anyhow::Ok;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use reqwest::{Method, header::{HeaderMap, HeaderName, HeaderValue, self}, Response};
use serde_json::json;
use url::Url;

pub use diff::{DiffConfig, ResponseProfile, DiffProfile};
pub use req::RequestConfig;

use crate::ExtraArgs;

pub trait ValidateConfig {
    fn validate(&self) -> anyhow::Result<()>;
}

#[async_trait]
pub trait LoadConfig
where 
    Self: Sized + ValidateConfig + DeserializeOwned
{
    /// load config from yaml file
    ///
    /// # Examples
    /// Deserializing a single document:
    ///
    /// ```
    /// use xdiff::DiffConfig;
    /// ```
    ///
    async fn load_yaml(path: &str) -> anyhow::Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }

    /// load config from yaml string
    fn from_yaml(content: &str) -> anyhow::Result<Self> {
        let config: Self = serde_yaml::from_str(content)?;
        config.validate()?;
        Ok(config)
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct RequestProfile {
    #[serde(with = "http_serde::method")]
    pub method: Method,
    pub url: Url,
    #[serde(skip_serializing_if = "empty_json_value", default)]
    pub params: Option<serde_json::Value>,
    #[serde(
        skip_serializing_if = "HeaderMap::is_empty",
        with = "http_serde::header_map",
        default
    )]
    pub headers: HeaderMap,
    #[serde(skip_serializing_if = "empty_json_value", default)]
    pub body: Option<serde_json::Value>,
}
 
impl RequestProfile {
    pub fn new(method: Method, url: Url, params: Option<serde_json::Value>, headers: HeaderMap, body: Option<serde_json::Value>) -> Self {
        Self {
            method,
            url,
            params,
            headers,
            body,
        }
    }

    pub async fn send(&self, args: &ExtraArgs) -> anyhow::Result<ResponseExt> {
        let (headers, query, body) = self.generate(args)?;
        let client = reqwest::Client::new();
        let req = client
            .request(self.method.clone(), self.url.clone())
            .headers(headers)
            .query(&query)
            .body(body)
            .build()?;
        let res = client.execute(req).await?;
        
        Ok(ResponseExt(res))
    }

    pub fn get_url(&self, args: &ExtraArgs) -> anyhow::Result<String> {
        let mut url = self.url.clone();
        let (_, params, _) = self.generate(args)?;
        if !params.as_object().unwrap().is_empty() {
            // url query转string
            let query = serde_qs::to_string(&params)?;
            url.set_query(Some(&query));
        }
        Ok(url.to_string())
    }

    pub fn generate(&self, args: &ExtraArgs) -> anyhow::Result<(HeaderMap, serde_json::Value, String)> {
        let mut headers = self.headers.clone();
        let mut query = self.params.clone().unwrap_or_else(|| serde_json::json!({}));
        let mut body = self.body.clone().unwrap_or_else(|| serde_json::json!({}));
        // 将client参数加入到headers
        for (k, v) in args.headers.iter() {
            headers.insert(HeaderName::from_str(k)?, HeaderValue::from_str(v)?);
        }
        if !headers.contains_key(header::CONTENT_TYPE) {
            headers.insert(header::CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        }

        for (k, v) in args.query.iter() {
            query[k] = v.parse()?;
        }

        for (k, v) in args.body.iter() {
            body[k] = v.parse()?;
        }

        let content_type = get_content_type(&headers);
        match content_type.as_deref() {
            Some("application/json") => {
                let body = serde_json::to_string(&body)?;
                Ok((headers, query, body))
            }
            Some("application/x-www-form-urlencoded" | "multipart/form-data") => {
                let body = serde_urlencoded::to_string(&body)?;
                Ok((headers, query, body))
            }
            _ => Err(anyhow::anyhow!("unsupported content-type")),
        }
    }
}

impl ValidateConfig for RequestProfile {
    fn validate(&self) -> anyhow::Result<()> {
        if let Some(params) = self.params.as_ref() {
            if !params.is_object() {
                return Err(anyhow::anyhow!("Params must be an object but got\n{}",
                serde_yaml::to_string(params)?));
            }
        }
        if let Some(body) = self.body.as_ref() {
            if !body.is_object() {
                return Err(anyhow::anyhow!("Body must be an object but got\n{}",
                serde_yaml::to_string(body)?));
            }
        }
        Ok(())
    }
}

impl FromStr for RequestProfile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut url = Url::parse(s)?;
        let qs = url.query_pairs();
        let mut params = json!({});

        for (k, v) in qs {
            params[k.to_string()] = v.parse()?;
        }

        url.set_query(None);
        let profile = RequestProfile {
            method: Method::GET,
            url,
            params: Some(params),
            headers: HeaderMap::new(),
            body: None,
        };

        Ok(profile)
    }
}

#[derive(Debug)]
pub struct ResponseExt(Response);

impl ResponseExt {
    pub fn into_inner(self) -> Response {
        self.0
    }

    pub async fn get_text(self, profile: &ResponseProfile) -> anyhow::Result<String> {
        let res = self.0;
        let mut output = get_status_text(&res)?;
        // version headers
        write!(
            &mut output,
            "{}",
            get_header_text(&res, &profile.skip_headers)?
        )?;

        // body
        let body = get_body_text(res, &profile.skip_body).await?;
        write!(&mut output, "{}", body)?;

        Ok(output)
    }

    pub fn get_header_keys(&self) -> Vec<String>{
        let resp = &self.0;
        let headers = resp.headers();
        let mut set = HashSet::new();
        headers
            .iter()
            .map(|(k, _)| k.as_str().to_string())
            .filter(|k| {
                if !set.contains(k.as_str()) {
                    set.insert(k.to_string());
                    return true;
                }
                false
            })
            .collect()
    }
}

// json value 为空 
pub fn empty_json_value(v: &Option<serde_json::Value>) -> bool {
  v.as_ref().map_or(true, |v| {
      v.is_null() || (v.is_object() && v.as_object().unwrap().is_empty())
  })
}

// 处理请求头中的content-type, e: application/json;charset=utf-8
fn get_content_type(headers: &HeaderMap) -> Option<String> {
  headers
      .get(header::CONTENT_TYPE)
      .and_then(|v| v.to_str().unwrap().split(';').next().map(|v| v.to_string()))
}

fn filter_json(text: &str, skips: &[String]) -> anyhow::Result<String> {
  let mut json: serde_json::Value = serde_json::from_str(text)?;
  // 如果json是对象，删除skip的值
  if let serde_json::Value::Object(ref mut v) = json {
      for k in skips {
          v.remove(k);
      }
  }

  Ok(serde_json::to_string_pretty(&json)?)
}

pub fn get_status_text(res: &Response) -> anyhow::Result<String> {
  Ok(format!("{:?} {}\n", res.version(), res.status()))
}

pub fn get_header_text(res: &Response, skip_headers: &[String]) -> anyhow::Result<String> {
  let mut output = String::new();
  let headers = res.headers();
  for (k, v) in headers.iter() {
      if !skip_headers.iter().any(|sh| sh == k.as_str()) {
          writeln!(&mut output, "{}: {:?}", k, v)?;
      }
  }
  writeln!(&mut output)?;
  Ok(output)
}

pub async fn get_body_text(res: Response, skip_body: &[String]) -> anyhow::Result<String> {
  let content_type = get_content_type(res.headers());
  let text = res.text().await?;
  match content_type.as_deref() {
      Some("application/json") => filter_json(&text, &skip_body),
      _ => Ok(text),
  }
}