use xdiff::{cli::{KeyVal, KeyValType}, ExtraArgs};

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
