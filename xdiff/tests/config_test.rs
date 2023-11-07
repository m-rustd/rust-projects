use xdiff::{DiffConfig, LoadConfig};

#[tokio::test]
async fn config_load_yaml() {
    let config = DiffConfig::load_yaml("fixtures/test.yaml").await.unwrap();
    let profile = config.get_profile("rust").unwrap();

    println!("{:#?}", profile);
    assert_eq!(profile.req1.method, "GET");
}