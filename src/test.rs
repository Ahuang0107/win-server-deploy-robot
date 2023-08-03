use std::fs::File;
use std::io::Write;

#[test]
fn test_config_file() {
    use super::util::*;
    use std::collections::HashMap;

    let config = Config {
        profiles: HashMap::from([
            (
                "<App01>".into(),
                ServerConfig {
                    host: "<host01>".into(),
                    username: "<usr01>".into(),
                    password: "<pwd01>".into(),
                    target_folder: "<folder1>".into(),
                    files: vec!["<path1>".into(), "<path3>".into()],
                },
            ),
            (
                "<App02>".into(),
                ServerConfig {
                    host: "<host02>".into(),
                    username: "<usr02>".into(),
                    password: "<pwd02>".into(),
                    target_folder: "<folder2>".into(),
                    files: vec!["<path3>".into(), "<path4>".into()],
                },
            ),
        ]),
    };
    let mut file = File::create("config.sample.yaml").unwrap();
    let content = serde_yaml::to_string(&config).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
