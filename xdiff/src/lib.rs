pub mod cli;
mod utils;
mod config;

pub use utils::{process_error_output, highlight_text, diff_text};
pub use config::DiffConfig;

// 提供解析参数给外部使用
#[derive(Debug, Default, PartialEq, Eq)]
pub struct ExtraArgs {
    pub headers: Vec<(String, String)>,
    pub query: Vec<(String, String)>,
    pub body: Vec<(String, String)>,
}

impl ExtraArgs {
    pub fn new_with_query(query: Vec<(String, String)>) -> Self {
        Self { 
            query,
            ..Default::default()
        }
    }
    pub fn new_with_headers(headers: Vec<(String, String)>) -> Self {
        Self { 
            headers,
            ..Default::default()
        }
    }
    pub fn new_with_body(body: Vec<(String, String)>) -> Self {
        Self { 
            body,
            ..Default::default()
        }
    }
}

impl From<Vec<cli::KeyVal>> for ExtraArgs {
    fn from(args: Vec<cli::KeyVal>) -> Self {
        let mut query = vec![];
        let mut headers = vec![];
        let mut body = vec![];

        for arg in args {
            match arg.key_type {
                cli::KeyValType::Header => headers.push((arg.key, arg.value)),
                cli::KeyValType::Query => query.push((arg.key, arg.value)),
                cli::KeyValType::Body => body.push((arg.key, arg.value)),
            }
        }
        Self { headers, query, body }
    }
}