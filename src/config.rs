#[derive(Debug)]
pub struct Config {
    pub n_threads: usize,
    pub input_file: Option<String>,
    pub encrypted_file: Option<String>,
    pub decrypted_file: Option<String>,
    pub repeat: usize,
    pub publish_metrics: bool
}

impl Config {
    pub fn new_from_env() -> Self {
        let n_threads = std::env::var("N_THREADS")
            .unwrap_or("1".to_string())
            .parse()
            .expect("Error while parsing N_THREADS");

        let input_file = std::env::var("PLAIN_TEXT").ok();
        let encrypted_file = std::env::var("ENCRYPTED_TEXT").ok();
        let decrypted_file = std::env::var("DECRYPTED_TEXT").ok();
        let repeat = std::env::var("REPEAT")
            .unwrap_or("1".to_string())
            .parse()
            .expect("Error while parsing REPEAT");
        let publish_metrics = std::env::var("LOCAL").unwrap_or("false".to_string()).as_str() == "true";

        Config {
            n_threads,
            input_file,
            encrypted_file,
            decrypted_file,
            repeat,
            publish_metrics
        }
    }
}