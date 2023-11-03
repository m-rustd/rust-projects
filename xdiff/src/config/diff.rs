use std::{collections::HashMap, str::FromStr};

use anyhow::Ok;
use reqwest::{Method, header::{HeaderMap, HeaderName, HeaderValue, self}, Response};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{ExtraArgs, diff_text};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffConfig {
    #[serde(flatten)]
    pub profiles: HashMap<String, DiffProfile>,
}

impl DiffConfig {
  /// 从yaml文件中加载配置
  ///
  /// # Examples
  /// Deserializing a single document:
  ///
  /// ```
  /// use xdiff::DiffConfig;
  /// ```
  ///
  pub async fn load_yaml(path: &str) -> anyhow::Result<Self> {
      let content = tokio::fs::read_to_string(path).await?;
      Self::from_yaml(content.as_str())
  }

  /// 从yaml字符串中加载配置
  pub fn from_yaml(content: &str) -> anyhow::Result<Self> {
      let config = serde_yaml::from_str(content)?;
      Ok(config)
  }

  pub fn get_profile(&self, name: &str) -> Option<&DiffProfile> {
      self.profiles.get(name)
  }
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiffProfile {
    pub req1: RequestProfile,
    pub req2: RequestProfile,
    #[serde(skip_serializing_if = "is_default", default)]
    pub res: ResponseProfile,
}

impl DiffProfile {
    pub async fn diff(&self, extra_args: &ExtraArgs) -> anyhow::Result<String> {
        let res1 = self.req1.send(extra_args).await?;
        let res2 = self.req2.send(extra_args).await?;

        let text1 = res1.get_text(&self.res).await?;
        let text2 = res2.get_text(&self.res).await?;

        // 对比两个文本
       let (output, _output1, _output2)  = diff_text(&text1, &text2)?;
       
       Ok(output)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}

#[derive(Debug)]
pub struct ResponseExt(Response);

impl ResponseExt {
    pub async fn get_text(self, profile: &ResponseProfile) -> anyhow::Result<String> {
        let res = self.0;
        let mut output = String::new();
        // version status
        output.push_str(format!("{:?} {}\n", res.version(), res.status()).as_str());

        // headers
        let headers = res.headers();
        for (k, v) in headers.iter() {
            if !profile.skip_headers.contains(&(k.as_str().to_string())) {
                output.push_str(format!("{}: {:?}\n", k, v).as_str());
            }
            // if !profile.skip_headers.iter().any(|sh| sh == k.as_str()) {
            //     writeln!(&mut output, "{}: {:?}", k, v)?;
            // }
        }
        // let body = get_body_text(res, &profile.skip_body).await?;
        let content_type = get_content_type(res.headers());
        let text: String = res.text().await?;
        let body = match content_type.as_deref() {
            Some("application/json") => filter_json(&text, &profile.skip_body),
            _ => Ok(text),
        };
        output.push_str(body?.as_str());

        Ok(output)
    }
}

pub fn is_default<T>(value: &T) -> bool
where
    T: Default + PartialEq,
{
    value == &T::default()
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