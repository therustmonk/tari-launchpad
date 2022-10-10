use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{Envs, ManagedContainer, Mounts, Networks, Ports, Volumes},
};

use super::GRAFANA_REGISTRY;
use crate::{
    config::{ConnectionSettings, LaunchpadConfig, LaunchpadProtocol},
    images::{GENERAL_VOLUME, GRAFANA_DEFAULTS_PATH, GRAFANA_PROVISION_PATH, GRAFANA_VOLUME, VAR_TARI_PATH},
    networks::LocalNet,
};
use crate::volumes::SharedGrafanaVolume;

#[derive(Debug, Default)]
pub struct Grafana {
    settings: Option<ConnectionSettings>,
}

impl ManagedTask for Grafana {
    fn id() -> TaskId {
        "Grafana".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![LocalNet::id(), SharedGrafanaVolume::id()]
    }
}

impl ManagedContainer for Grafana {
    type Protocol = LaunchpadProtocol;

    fn registry(&self) -> &str {
        GRAFANA_REGISTRY
    }

    fn image_name(&self) -> &str {
        "grafana"
    }

    fn envs(&self, envs: &mut Envs) {
        let path = concat!(
            "/usr/share/grafana/bin:",
            "/usr/local/sbin:",
            "/usr/local/bin:",
            "/usr/sbin:",
            "/usr/bin:",
            "/sbin:",
            "/bin"
        );
        envs.set("PATH", path);
        if let Some(settings) = self.settings.as_ref() {
            // TODO: Check the `display` call is correct here?
            envs.set("DATA_FOLDER", settings.data_directory.display());
        }
    }

    fn networks(&self, networks: &mut Networks) {
        networks.add("grafana", LocalNet::id());
    }

    fn ports(&self, ports: &mut Ports) {
        ports.add(18_300);
    }

    fn reconfigure(&mut self, config: Option<&LaunchpadConfig>) -> bool {
        self.settings = config.map(ConnectionSettings::from);
        config.map(|conf| conf.with_monitoring).unwrap_or_default()
    }

    fn volumes(&self, volumes: &mut Volumes) {
        volumes.add(GENERAL_VOLUME);
    }

    fn mounts(&self, mounts: &mut Mounts) {
        mounts.add_volume(SharedGrafanaVolume::id(), GRAFANA_VOLUME);
        if let Some(settings) = self.settings.as_ref() {
            // TODO: Avoid using display here
            mounts.bind_path(settings.data_directory.display(), VAR_TARI_PATH);
            mounts.bind_path(
                settings.data_directory.join("config").join("defaults.ini").display(),
                GRAFANA_DEFAULTS_PATH,
            );
            mounts.bind_path(
                settings
                    .data_directory
                    .join("config")
                    .join("sources_provision.yml")
                    .display(),
                GRAFANA_PROVISION_PATH,
            );
        }
    }
}
