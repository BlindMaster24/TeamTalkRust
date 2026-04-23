//! Hook registrations for audio/video/desktop media events, transfer
//! events, and voice-activation / hotkey events.

use crate::client::hooks::ClientHooks;
use crate::client::{Client, Message};

impl ClientHooks {
    /// Registers a handler for video capture frames.
    #[must_use]
    pub fn on_video_capture_frame(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_video_capture_frame = Some(Box::new(hook));
        self
    }

    /// Registers a handler for media file video frames.
    #[must_use]
    pub fn on_media_file_video(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_media_file_video = Some(Box::new(hook));
        self
    }

    /// Registers a handler for desktop window updates.
    #[must_use]
    pub fn on_desktop_window(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_desktop_window = Some(Box::new(hook));
        self
    }

    /// Registers a handler for desktop cursor updates.
    #[must_use]
    pub fn on_desktop_cursor(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_desktop_cursor = Some(Box::new(hook));
        self
    }

    /// Registers a handler for desktop input updates.
    #[must_use]
    pub fn on_desktop_input(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_desktop_input = Some(Box::new(hook));
        self
    }

    /// Registers a handler for recorded media file events.
    #[must_use]
    pub fn on_user_record_media_file(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_user_record_media_file = Some(Box::new(hook));
        self
    }

    /// Registers a handler for audio block events.
    #[must_use]
    pub fn on_audio_block(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_audio_block = Some(Box::new(hook));
        self
    }

    /// Registers a handler for voice activation events.
    #[must_use]
    pub fn on_voice_activation(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_voice_activation = Some(Box::new(hook));
        self
    }

    /// Registers a handler for hotkey events.
    #[must_use]
    pub fn on_hotkey(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_hotkey = Some(Box::new(hook));
        self
    }

    /// Registers a handler for hotkey test events.
    #[must_use]
    pub fn on_hotkey_test(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_hotkey_test = Some(Box::new(hook));
        self
    }

    /// Registers a handler for file transfer events.
    #[must_use]
    pub fn on_file_transfer(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_file_transfer = Some(Box::new(hook));
        self
    }

    /// Registers a handler for desktop window transfer events.
    #[must_use]
    pub fn on_desktop_window_transfer(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_desktop_window_transfer = Some(Box::new(hook));
        self
    }

    /// Registers a handler for stream media file events.
    #[must_use]
    pub fn on_stream_media_file(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_stream_media_file = Some(Box::new(hook));
        self
    }

    /// Registers a handler for local media file events.
    #[must_use]
    pub fn on_local_media_file(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_local_media_file = Some(Box::new(hook));
        self
    }

    /// Registers a handler for audio input events.
    #[must_use]
    pub fn on_audio_input(mut self, hook: impl FnMut(&Client, &Message) + Send + 'static) -> Self {
        self.on_audio_input = Some(Box::new(hook));
        self
    }

    /// Registers a handler for first voice stream packet events.
    #[must_use]
    pub fn on_user_first_voice_stream_packet(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_user_first_voice_stream_packet = Some(Box::new(hook));
        self
    }

    /// Registers a handler for sound device added events.
    #[must_use]
    pub fn on_sound_device_added(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_sound_device_added = Some(Box::new(hook));
        self
    }

    /// Registers a handler for sound device removed events.
    #[must_use]
    pub fn on_sound_device_removed(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_sound_device_removed = Some(Box::new(hook));
        self
    }

    /// Registers a handler for sound device unplugged events.
    #[must_use]
    pub fn on_sound_device_unplugged(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_sound_device_unplugged = Some(Box::new(hook));
        self
    }

    /// Registers a handler for default sound input device changes.
    #[must_use]
    pub fn on_sound_device_new_default_input(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_sound_device_new_default_input = Some(Box::new(hook));
        self
    }

    /// Registers a handler for default sound output device changes.
    #[must_use]
    pub fn on_sound_device_new_default_output(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_sound_device_new_default_output = Some(Box::new(hook));
        self
    }

    /// Registers a handler for default sound input communications device changes.
    #[must_use]
    pub fn on_sound_device_new_default_input_com_device(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_sound_device_new_default_input_com_device = Some(Box::new(hook));
        self
    }

    /// Registers a handler for default sound output communications device changes.
    #[must_use]
    pub fn on_sound_device_new_default_output_com_device(
        mut self,
        hook: impl FnMut(&Client, &Message) + Send + 'static,
    ) -> Self {
        self.on_sound_device_new_default_output_com_device = Some(Box::new(hook));
        self
    }
}
