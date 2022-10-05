mod task;

use std::fmt;

pub(crate) use task::VolumeTask;

use crate::config::ManagedProtocol;

pub trait ManagedVolume: fmt::Debug + Send + 'static {
    type Config: ManagedProtocol;

    fn reconfigure(&mut self, config: Option<&Self::Config>) -> bool {
        // Start if config exists
        config.is_some()
    }

    fn volume_name(&self) -> &str;
}
