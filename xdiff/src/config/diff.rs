use std::collections::HashMap;

use anyhow::{Ok, Context};
use serde::{Deserialize, Serialize};

use crate::{ExtraArgs, diff_text};
use super::{LoadConfig, RequestProfile, ValidateConfig};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffConfig {
    #[serde(flatten)]
    pub profiles: HashMap<String, DiffProfile>,
}

impl LoadConfig for DiffConfig {}

impl DiffConfig {
  pub fn new(profiles: HashMap<String, DiffProfile>) -> Self {
      Self { profiles }
  }
  
  pub fn get_profile(&self, name: &str) -> Option<&DiffProfile> {
      self.profiles.get(name)
  }
}

impl ValidateConfig for DiffConfig {
  fn validate(&self) -> anyhow::Result<()> {
      for (name, profile) in &self.profiles {
          profile
              .validate()
              .context(format!("failed to validate profile: {}", name))?;
      }
      Ok(())
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
    pub fn new(req1: RequestProfile, req2: RequestProfile, res: ResponseProfile) -> Self {
        Self { req1, req2, res }
    }

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

impl ValidateConfig for DiffProfile {
  fn validate(&self) -> anyhow::Result<()> {
      self.req1.validate().context("req1 failed to validate")?;
      self.req2.validate().context("req2 failed to validate")?;
      Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}

impl ResponseProfile {
    pub fn new(skip_headers: Vec<String>, skip_body: Vec<String>) -> Self {
        Self { skip_headers, skip_body }
    }
    
}

pub fn is_default<T>(value: &T) -> bool
where
    T: Default + PartialEq,
{
    value == &T::default()
}

