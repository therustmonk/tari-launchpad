use super::mining::{MergedMiningInfo, TariMiningInfo};
use super::onboarding::Onboarding;
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct LaunchpadState {
    pub onboarding: Onboarding,
    pub tari_mining: TariMiningInfo,
    pub merged_mining: MergedMiningInfo,
}

impl LaunchpadState {
    pub fn new() -> Self {
        let tari_mining = TariMiningInfo {
            mining_started: None,
            tari_amount: 123_456.into(),
        };
        let merged_mining = MergedMiningInfo {
            mining_started: None,
            tari_amount: 45_000.into(),
            monero_amount: Decimal::new(35, 1),
        };
        let onboarding = Onboarding::default();
        Self {
            onboarding,
            tari_mining,
            merged_mining,
        }
    }

    pub fn update(&mut self, delta: LaunchpadDelta) {
        match delta {}
    }
}

#[derive(Debug, Clone)]
pub enum LaunchpadAction {}

#[derive(Debug, Clone)]
pub enum LaunchpadDelta {}