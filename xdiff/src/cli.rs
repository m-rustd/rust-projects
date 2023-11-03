use anyhow::{anyhow, Ok, Result};
use clap::{Parser, Subcommand};

/// 比较两个http请求响应的不同
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
#[non_exhaustive] // 还有其他扩展，实现需要加"- => {}"配置
pub enum Action {
    /// 根据给定的profile对比两个api返回的差异
    Run(RunArgs),
    /// 解析URLs并生成profile
    Parse,
}

#[derive(Parser, Debug)]
pub struct RunArgs {
    /// 请求profile名称
    #[clap(short, long)]
    pub profile: String,

    /// headers、query和body参数解析
    /// query, e: `-e key=value`
    /// header, e: `-e %key=value`
    /// body, e: `-e @key=value`
    #[clap(short, long, value_parser = perse_key_val, number_of_values = 1)]
    pub extra_params: Vec<KeyVal>,

    /// 配置文件
    #[clap(short, long)]
    pub config: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum KeyValType {
    Header,
    Query,
    Body,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct KeyVal {
    pub key_type: KeyValType,
    pub key: String,
    pub value: String,
}

fn perse_key_val(s: &str) -> Result<KeyVal> {
    let mut parts = s.splitn(2, "=");
    let key = parts
        .next()
        .ok_or_else(|| anyhow!("Invalid key value pair(key parse)"))?
        .trim();
    let value = parts
        .next()
        .ok_or_else(|| anyhow!("Invalid key value pair(key parse)"))?
        .trim();

    let (key_type, key) = match key.chars().next() {
        Some('%') => (KeyValType::Header, &key[1..]),
        Some('@') => (KeyValType::Body, &key[1..]),
        Some(v) if v.is_ascii_alphabetic() => (KeyValType::Query, key),
        _ => return Err(anyhow!("Invalid key value pair")),
    };

    Ok(KeyVal {
        key_type,
        key: key.to_string(),
        value: value.to_string(),
    })
}

#[cfg(test)]
mod cli_tests {
    use super::*;
    #[test]
    fn parse_key_val_should_work() {
        let args = vec!["%key1=value1", "key2=value2", "@key3=value3", "key4=value4"];

        let key_vals = args
            .into_iter()
            .map(|arg| perse_key_val(arg))
            .collect::<Result<Vec<_>>>()
            .unwrap();

        assert_eq!(
            key_vals,
            vec![
                KeyVal {
                    key_type: KeyValType::Header,
                    key: "key1".into(),
                    value: "value1".into(),
                },
                KeyVal {
                    key_type: KeyValType::Query,
                    key: "key2".into(),
                    value: "value2".into()
                },
                KeyVal {
                    key_type: KeyValType::Body,
                    key: "key3".into(),
                    value: "value3".into()
                },
                KeyVal {
                    key_type: KeyValType::Query,
                    key: "key4".into(),
                    value: "value4".into()
                }
            ]
        )
    }
}
