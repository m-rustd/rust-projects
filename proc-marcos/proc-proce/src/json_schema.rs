use std::{fs, collections::HashMap};

use anyhow::{Result, Ok};
use askama::Template;
use heck::{AsSnakeCase, AsPascalCase};
use proc_macro::TokenStream;
use litrs::Literal;
use serde::{Serialize, Deserialize};

#[derive(Template)]
#[template(path = "code.j2")]
pub struct StructsTemplate {
    structs: Vec<St>
}

impl StructsTemplate {
    pub fn try_new(filename: &str) -> Result<Self> {
        let content = fs::read_to_string(filename)?;
        let schema: Schema = serde_json::from_str(&content)?;
        Ok(Self { 
          structs: schema.into_vec_st(),
        })
    }

    pub fn render(filename: &str) -> Result<String> {
        let template = Self::try_new(filename)?;
        Ok(template.render()?)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Schema {
    title: Option<String>,
    #[serde(rename = "type")]
    ty: String,
    properties: Option<HashMap<String, Schema>>,
}

impl Schema {
    pub fn into_vec_st(&self) -> Vec<St> {
        let mut structs = vec![];
        match self.ty.as_str() {
            "object" => {
                let fields = self.properties
                    .as_ref()
                    .unwrap()
                    .into_iter()
                    .map(|(k, v)| process_type(&mut structs, k.as_str(), &v))
                    .collect();
                structs.push(St::new(p(self.title.as_ref().unwrap()), fields));
                structs
            },
            _ => panic!("Not supported type: {}", self.ty),
        }
    }
}

pub struct St {
    name: String,
    fields: Vec<Fd>,
}

impl St {
    pub fn new(name: impl Into<String>, fields: Vec<Fd>) -> Self {
        Self { name: name.into(), fields }
    }
}

pub struct Fd {
    name: String,
    ty: String,
}

impl Fd {
    pub fn new(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Self { name: name.into(), ty: ty.into() }
    }
}

// properties schema -> Fd
fn process_type(structs: &mut Vec<St>, k: &str, v: &Schema) -> Fd {
    let name = n(k);
    match v.ty.as_str() {
        "object" => {
            let sts = v.into_vec_st();
            structs.extend(sts);
            Fd::new(name, gen_name(v.title.as_deref(), k))
        },
        "integer" => Fd::new(name, "i64"),
        "float" => Fd::new(name, "f64"),
        "string" => Fd::new(name, "String"),
        v => panic!("Not supported type: {}", v),
    }
}

fn gen_name(first: Option<&str>, second: &str) -> String {
    p(first.unwrap_or(second))
}

// 生成下划线变量
fn n(s: &str) -> String {
    AsSnakeCase(s).to_string()
}

// 生成大驼峰
fn p(s: &str) -> String {
    AsPascalCase(s).to_string()
}

pub fn get_string_literal(input: TokenStream) -> Result<String> {
    input.into_iter()
        .next()
        .and_then(|v| Literal::try_from(v).ok())
        .and_then(|v| match v {
            Literal::String(s) => Some(s.value().to_string()),
            _ => None,
        })
        .ok_or_else(|| anyhow::anyhow!("Only string literals are allowed"))
}