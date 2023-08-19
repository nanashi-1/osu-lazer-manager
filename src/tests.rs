#[allow(dead_code)]
mod config_tests {
    use std::fs::create_dir_all;

    use crate::config::Config;

    fn init_config() -> Config {
        create_dir_all("test-files").unwrap();

        Config {
            version: "Not Installed".into(),
            config_path: "test-files/.cfg".into(),
        }
    }

    #[test]
    fn normal_use() {
        let mut config = init_config();

        config.version = "Updated Version".into();
        config.save().unwrap();
    }
}

mod error_tests {}

#[allow(dead_code)]
mod fetcher_test {
    use std::fs::create_dir_all;

    use crate::{fetcher::Fetcher, Context};

    fn init_fetcher() -> Fetcher {
        create_dir_all("test-files").unwrap();

        Fetcher::new(
            "http://speedtest.ftp.otenet.gr/files/test100k.db",
            "test-files/.url",
            Context {
                force: true,
                ..Default::default()
            },
        )
    }

    #[tokio::test]
    async fn normal_use() {
        let fetcher = init_fetcher();

        fetcher.fetch().await.unwrap();
    }
}

mod path_tests {
    #[allow(unused_imports)]
    use crate::paths::{get_path, Paths};

    #[test]
    fn normal_use() {
        assert_eq!(
            get_path(Paths::AppImage).unwrap(),
            dirs::data_dir()
                .unwrap()
                .join("osu-lazer-manager/osu.AppImage")
        )
    }
}

mod printer_tests {
    #[allow(unused_imports)]
    use crate::printer::Printer;

    #[test]
    fn normal_use() {
        "Error".print_as_error();
        "Warning".print_as_warning();
        "Success".print_as_success();
        "Normal".print();
    }
}
