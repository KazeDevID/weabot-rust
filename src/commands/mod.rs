pub mod general;
pub mod group;
pub mod media;

use std::time::Instant;
use whatsapp_rust::prelude::*;

pub fn register_all(router: &mut BotRouter, start_time: Instant) {
    general::register(router, start_time);
    group::register(router);
    media::register(router);
}
