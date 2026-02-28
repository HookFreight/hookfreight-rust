mod config;
mod error;
mod http_client;
mod pagination;
mod services;

pub use config::HookfreightConfig;
pub use error::{ApiError, HookfreightError};
pub use services::{
    apps::AppsService, deliveries::DeliveriesService, endpoints::EndpointsService,
    events::EventsService,
};

use http_client::HFHttpClient;

#[derive(Clone)]
pub struct Hookfreight {
    pub apps: AppsService,
    pub endpoints: EndpointsService,
    pub events: EventsService,
    pub deliveries: DeliveriesService,
}

impl Hookfreight {
    pub fn new(config: HookfreightConfig) -> Result<Self, HookfreightError> {
        let http = HFHttpClient::new(config)?;

        Ok(Self {
            apps: AppsService::new(http.clone()),
            endpoints: EndpointsService::new(http.clone()),
            events: EventsService::new(http.clone()),
            deliveries: DeliveriesService::new(http),
        })
    }
}
