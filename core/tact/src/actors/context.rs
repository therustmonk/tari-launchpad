use super::action::Do;
use super::actor::Actor;
use super::address::{Address, SendError};
use super::joint::{ActorState, AddressJoint};
use tokio::sync::{mpsc, watch};

pub struct ActorContext<A: Actor> {
    address: Address<A>,
    joint: AddressJoint<A>,
}

impl<A: Actor> ActorContext<A> {
    pub(super) fn new() -> Self {
        let (tx_event, rx_event) = mpsc::unbounded_channel();
        let (tx_state, rx_state) = watch::channel(ActorState::Active);
        let joint = AddressJoint::new(rx_event, tx_state);
        let address = Address::new(tx_event, rx_state);
        Self { address, joint }
    }

    pub fn address(&self) -> &Address<A> {
        &self.address
    }

    pub(crate) fn joint(&mut self) -> &mut AddressJoint<A> {
        &mut self.joint
    }

    pub fn do_next<E>(&self, action: E) -> Result<(), SendError>
    where
        A: Do<E>,
        E: Send + 'static,
    {
        self.address.send(action)
    }

    pub fn shutdown(&mut self) {
        self.joint.close();
    }
}
