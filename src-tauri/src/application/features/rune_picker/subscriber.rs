use crate::application::features::rune_picker::request::RunePickRequest;
use crate::application::features::rune_picker::service::RunePickerService;
use crate::domain::delay_state::DelayState;
use crate::domain::events::{EventType, LeagueEvent};
use crate::domain::ports::{ChampionResolverPort, LcuApiPort, LeagueEventSubscriber};
use crate::domain::rune_page_manager::RunePageManager;
use log::{debug, info};
use std::sync::Arc;
use tokio::task;
use tokio::time::{sleep, Duration};

pub struct RunePickerSubscriber {
    api: Arc<dyn LcuApiPort>,
    delay: Arc<DelayState>,
    pages: Arc<RunePageManager>,
    resolver: Arc<dyn ChampionResolverPort>,
}

impl RunePickerSubscriber {
    pub fn new(
        api: Arc<dyn LcuApiPort>,
        delay: Arc<DelayState>,
        pages: Arc<RunePageManager>,
        resolver: Arc<dyn ChampionResolverPort>,
    ) -> Arc<Self> {
        Arc::new(Self {
            api,
            delay,
            pages,
            resolver,
        })
    }
}

impl LeagueEventSubscriber for RunePickerSubscriber {
    fn on_event(&self, event: &LeagueEvent) {
        if event.event_type != EventType::ChampSelectSession {
            return;
        }

        let session = event.data.clone();
        info!("RunePicker: session={:?}", session);

        let phase = session
            .get("timer")
            .and_then(|v| v.get("phase"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if phase == "PLANNING" {
            return;
        }

        let delay = self.delay.get();
        let api = self.api.clone();
        let pages = self.pages.clone();
        let resolver = self.resolver.clone();

        info!("RunePicker: scheduling rune evaluation in {} ms", delay);

        task::spawn(async move {
            sleep(Duration::from_millis(delay)).await;

            let req = RunePickRequest::from_session(&session, resolver.as_ref()).await;
            if req.is_none() {
                return;
            }

            let req = req.unwrap();
            let svc = RunePickerService::new(api, pages);
            svc.apply(req).await;
        });
    }
}
