use darling::FromField;
use proc_macro2::{Ident, TokenStream};
use syn::{Type, DeriveInput, Data, DataStruct, Fields, FieldsNamed, Field, TypePath, Path, GenericArgument};
use quote::quote;

pub struct BuilderContext {
    name: Ident,
    fields: Vec<Fd>,
}

struct Fd {
    name: Ident,
    ty: Type,
    optional: bool,
    opts: Opts,
}

#[derive(Debug, Default, FromField)]
#[darling(default, attributes(builder))]
struct Opts {
    each: Option<String>,
    default: Option<String>,
}


impl From<Field> for Fd {
  fn from(field: Field) -> Self {
      let (optional, ty) = get_option_inner(&field.ty);
      // 从 Field 中读取 attributes 生成 Opts，如果没有使用缺省值
      let opts = Opts::from_field(&field).unwrap_or_default();
      Self {
          name: field.ident.unwrap(),
          ty: ty.to_owned(),
          optional,
          opts,
      }
  }
}

impl From<DeriveInput> for BuilderContext {
  fn from(input: DeriveInput) -> Self {
      let name = input.ident;
      let fields = if let Data::Struct(DataStruct {
          fields: Fields::Named(FieldsNamed { named, .. }),
          ..
      }) = input.data
      {
          named
      } else {
          panic!("Unsupported data type");
      };
      let fds = fields.into_iter().map(Fd::from).collect();
      Self { name, fields: fds }
  }
}


impl BuilderContext {
  pub fn render(&self) -> TokenStream {
      let name = &self.name;
      // builder name: {}Builder, e.g. CommandBuilder
      let builder_name = Ident::new(&format!("{}Builder", name), name.span());
      // optional fields. e.g. executable: String -> executable: Option<String>,
      let optionized_fields = self.gen_optionized_fields();
      // methods: fn executable(mut self, v: impl Into<String>) -> Self { self.executable =
      // Some(v); self } Command::builder().executable("hello").args(vec![]).envs(vec![]).
      // finish()
      let methods = self.gen_methods();
      // assign Builder fields back to original struct fields
      // #field_name: self.#field_name.take().ok_or("xxx need to be set!")
      let assigns = self.gen_assigns();
      quote! {
          // Builder structure
          #[derive(Debug, Default)]
          struct #builder_name {
              #(#optionized_fields),*
          }

          impl #builder_name {
              #(#methods)*

              pub fn finish(mut self) -> Result<#name, &'static str> {
                  Ok(#name {
                      #(#assigns),*
                  })
              }
          }

          impl #name {
              pub fn builder() -> #builder_name {
                  Default::default()
              }
          }
      }
  }
  // 将属性变成可选
  fn gen_optionized_fields(&self) -> Vec<TokenStream> {
      self.fields
          .iter()
          .map(|Fd { name, ty, ..}| {
              quote! {
                  #name: std::option::Option<#ty>
              }
          })
          .collect()
  }
  // 生成属性方法
  fn gen_methods(&self) -> Vec<TokenStream> {
      self.fields
          .iter()
          .map(|Fd { name, ty, optional, opts }| {
              // 如果不是 Option 类型，且定义了 each attribute
              if !*optional && opts.each.is_some() {
                  let each = Ident::new(opts.each.as_deref().unwrap(), name.span());
                  let (is_vec, ty) = get_vec_inner(ty);
                  if is_vec {
                      return quote! {
                          pub fn #each(mut self, v: impl Into<#ty>) -> Self {
                              let mut data = self.#name.take().unwrap_or_default();
                              data.push(v.into());
                              self.#name = Some(data);
                              self
                          }
                      }
                  }
              }
              quote! {
                  pub fn #name(mut self, v: impl Into<#ty>) -> Self {
                      self.#name = Some(v.into());
                      self
                  }
              }
          })
          .collect()
  }
  // option属性生成take
  fn gen_assigns(&self) -> Vec<TokenStream> {
      self.fields
          .iter()
          .map(|Fd { name, optional, opts, .. }| {
              if *optional {
                  return quote! {
                      #name: self.#name.take()
                  };
              }
              // 如果定义了 default，那么把 default 里的字符串转换成 TokenStream
              // 使用 unwrap_or_else 在没有值的时候，使用缺省的结果
              if let Some(default) = opts.default.as_ref() {
                  let ast: TokenStream = default.parse().unwrap();
                  return quote! {
                      #name: self.#name.take().unwrap_or_else(|| #ast)
                  }
              }
              quote! {
                  #name: self.#name.take().ok_or(concat!(stringify!(#name), " needs to be set!"))?
              }
          })
          .collect()
  }
}

// 如果是 T = Option<Inner>，返回 (true, Inner)；否则返回 (false, T)
fn get_option_inner(ty: &Type) -> (bool, &Type) {
    get_type_inner(ty, "Option")
}

// 如果是 T = Vec<Inner>，返回 (true, Inner)；否则返回 (false, T)
fn get_vec_inner(ty: &Type) -> (bool, &Type) {
    get_type_inner(ty, "Vec")
}

fn get_type_inner<'a>(ty: &'a Type, name: &str) -> (bool, &'a Type) {
  if let Type::Path(TypePath {
      path: Path { segments, .. },
      ..
  }) = ty {
      if let Some(v) = segments.iter().next() {
          if v.ident == name {
              // 如果 PathSegment 第一个是 Option/Vec 等类型，那么它内部应该是
              // AngleBracketed，比如 <T> 获取其第一个值，如果是
              // GenericArgument::Type，则返回
              let t = match &v.arguments {
                  syn::PathArguments::AngleBracketed(a) => match a.args.iter().next() {
                      Some(GenericArgument::Type(t)) => t,
                      _ => panic!("Not sure what to do with other GenericArgument"),
                  },
                  _ => panic!("Not sure what to do with other PathArgument"),
              };
              return (true, t);
          }
      }
  }
  return (false, ty);
}