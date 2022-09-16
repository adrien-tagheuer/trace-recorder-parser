use crate::streaming::event::EventCount;
use crate::time::Timestamp;
use crate::types::ObjectHandle;
use derive_more::Display;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Display)]
#[display(fmt = "[{timestamp}]:{handle}:{queue_length}")]
pub struct QueueCreateEvent {
    pub event_count: EventCount,
    pub timestamp: Timestamp,

    pub handle: ObjectHandle,
    pub queue_length: u32,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Display)]
#[display(fmt = "[{timestamp}]:{handle}:{messages_waiting}")]
pub struct QueueEvent {
    pub event_count: EventCount,
    pub timestamp: Timestamp,

    pub handle: ObjectHandle,
    pub ticks_to_wait: Option<u32>,
    pub messages_waiting: u32,
}

pub type QueueSendEvent = QueueEvent;
pub type QueueSendBlockEvent = QueueEvent;
pub type QueueSendFromIsrEvent = QueueEvent;
pub type QueueSendFrontEvent = QueueEvent;
pub type QueueSendFrontBlockEvent = QueueEvent;
pub type QueueSendFrontFromIsrEvent = QueueEvent;
pub type QueueReceiveEvent = QueueEvent;
pub type QueueReceiveBlockEvent = QueueEvent;
pub type QueueReceiveFromIsrEvent = QueueEvent;
pub type QueuePeekEvent = QueueEvent;
pub type QueuePeekBlockEvent = QueueEvent;
