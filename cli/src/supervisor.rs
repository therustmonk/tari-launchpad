use crate::dashboard::{Dashboard, DashboardEvent};
use crate::onboarding::OnboardingWorker;
use crate::state::bus::Bus;
use anyhow::Error;
use async_trait::async_trait;
use tact::actors::{Actor, ActorContext, Address, Do};

pub struct Supervisor {
    dashboard: Option<Address<Dashboard>>,
    onboarding: Option<Address<OnboardingWorker>>,
    bus: Option<Bus>,
}

impl Supervisor {
    pub fn new() -> Self {
        Self {
            dashboard: None,
            onboarding: None,
            bus: None,
        }
    }
}

#[async_trait]
impl Actor for Supervisor {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        let bus = Bus::new();
        let dashboard = Dashboard::new(bus.clone(), ctx.recipient());
        let addr = dashboard.start();
        self.dashboard = Some(addr);
        let onboarding = OnboardingWorker::new(bus.clone());
        let addr = onboarding.start();
        self.onboarding = Some(addr);
        self.bus = Some(bus);
        Ok(())
    }
}

#[async_trait]
impl Do<DashboardEvent> for Supervisor {
    async fn handle(
        &mut self,
        event: DashboardEvent,
        ctx: &mut ActorContext<Self>,
    ) -> Result<(), Error> {
        match event {
            DashboardEvent::Terminated => {
                self.dashboard.take();
                ctx.shutdown();
            }
        }
        Ok(())
    }
}