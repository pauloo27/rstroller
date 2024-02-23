use mpris::Event;

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub metadata: mpris::Metadata,
    pub playback_status: mpris::PlaybackStatus,
    pub loop_status: mpris::LoopStatus,
    pub shuffle: bool,
    pub volume: f64,
}

impl PlayerState {
    pub fn new(player: &mpris::Player) -> Self {
        PlayerState {
            metadata: player.get_metadata().unwrap_or_default(),
            playback_status: player
                .get_playback_status()
                .unwrap_or(mpris::PlaybackStatus::Stopped),
            loop_status: player.get_loop_status().unwrap_or(mpris::LoopStatus::None),
            shuffle: false,
            volume: player.get_volume().unwrap_or(1.0),
        }
    }

    pub fn handle_event(mut self, event: mpris::Event) -> Self {
        match event {
            // the easy ones :)
            Event::TrackChanged(metadata) => self.metadata = metadata,
            Event::Playing => self.playback_status = mpris::PlaybackStatus::Playing,
            Event::Paused => self.playback_status = mpris::PlaybackStatus::Paused,
            Event::Stopped => self.playback_status = mpris::PlaybackStatus::Stopped,
            Event::LoopingChanged(status) => self.loop_status = status,
            Event::ShuffleToggled(status) => self.shuffle = status,
            Event::VolumeChanged(v) => self.volume = v,

            // seek is not reliable, instead we query it
            Event::Seeked { position_in_us: _ } => {}

            // TODO: need to handle those
            Event::PlayerShutDown => {}

            // we don't really care about the Tracklist, just the current one
            // if the track changed is the current one, will it call
            // TrackChanged? i do hope so, otherwise i'm screwed
            Event::TrackAdded(_)
            | Event::TrackRemoved(_)
            | Event::TrackMetadataChanged {
                old_id: _,
                new_id: _,
            }
            | Event::TrackListReplaced => {}

            // i can't do much with this
            Event::PlaybackRateChanged(_) => {}
        }
        self
    }
}
