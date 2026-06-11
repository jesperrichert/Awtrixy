use crate::Config;
use reqwest::Client;

pub struct Warptrixy {
    pub config: Config,
    pub(crate) client: Client,
}

impl Warptrixy {
    pub fn new(config: Config) -> Self {
        Warptrixy {
            config,
            client: Client::new(),
        }
    }
}


