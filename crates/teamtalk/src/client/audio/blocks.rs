use super::*;

impl Client {
    pub fn enable_audio_block_event(&self, user_id: UserId, types: u32, enable: bool) -> bool {
        unsafe {
            ffi::api().TT_EnableAudioBlockEvent(
                self.ptr.0,
                user_id.raw(),
                types,
                if enable { 1 } else { 0 },
            ) == 1
        }
    }

    /// Sets jitter control for a user.
    pub fn set_user_jitter_control(&self, user_id: UserId, delay: i32) -> bool {
        let mut cfg = unsafe { std::mem::zeroed::<ffi::JitterConfig>() };
        cfg.nFixedDelayMSec = delay;
        unsafe {
            ffi::api().TT_SetUserJitterControl(
                self.ptr.0,
                user_id.raw(),
                ffi::StreamType::STREAMTYPE_VOICE,
                &cfg,
            ) == 1
        }
    }

    /// Enables audio block events with a custom format.
    pub fn enable_audio_block_event_ex(
        &self,
        user_id: UserId,
        types: u32,
        format: Option<&ffi::AudioFormat>,
        enable: bool,
    ) -> bool {
        let fmt_ptr = format.map_or(std::ptr::null(), |f| f);
        unsafe {
            ffi::api().TT_EnableAudioBlockEventEx(
                self.ptr.0,
                user_id.raw(),
                types,
                fmt_ptr,
                if enable { 1 } else { 0 },
            ) == 1
        }
    }

    /// Subscribes to audio blocks for a user and streams them into the sink.
    pub fn stream_audio_blocks<S>(
        &self,
        user_id: UserId,
        types: u32,
        sink: S,
    ) -> AudioBlockSubscription<'_>
    where
        S: AudioBlockSink + Send + 'static,
    {
        // Register callback first to avoid dropping initial blocks between
        // event enable and handler subscription.
        let subscription = self.subscribe_audio_blocks(user_id, types, sink);
        let _ = self.enable_audio_block_event(user_id, types, true);
        subscription
    }

    /// Subscribes to audio blocks with a custom format.
    pub fn stream_audio_blocks_ex<S>(
        &self,
        user_id: UserId,
        types: u32,
        format: Option<&ffi::AudioFormat>,
        sink: S,
    ) -> AudioBlockSubscription<'_>
    where
        S: AudioBlockSink + Send + 'static,
    {
        // Register callback first to avoid dropping initial blocks between
        // event enable and handler subscription.
        let subscription = self.subscribe_audio_blocks(user_id, types, sink);
        let _ = self.enable_audio_block_event_ex(user_id, types, format, true);
        subscription
    }

    #[allow(clippy::too_many_arguments)]
    /// Starts a sound loopback test with additional options.
    pub fn start_sound_loopback_test_ex(
        &self,
        in_id: SoundDeviceId,
        out_id: SoundDeviceId,
        rate: i32,
        chans: i32,
        duplex: bool,
        preprocessor: Option<&AudioPreprocessor>,
        effects: Option<&ffi::SoundDeviceEffects>,
    ) -> *mut ffi::TTSoundLoop {
        let prep_raw = preprocessor.map(|p| p.to_ffi());
        let prep_ptr = prep_raw.as_ref().map_or(std::ptr::null(), |p| p);
        let eff_ptr = effects.map_or(std::ptr::null(), |e| e);
        unsafe {
            ffi::api().TT_StartSoundLoopbackTestEx(
                in_id.raw(),
                out_id.raw(),
                rate,
                chans,
                if duplex { 1 } else { 0 },
                prep_ptr,
                eff_ptr,
            )
        }
    }

    /// Returns jitter control settings for a user.
    pub fn get_user_jitter_control(&self, user_id: UserId) -> Option<crate::types::JitterConfig> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::JitterConfig>() };
        if unsafe {
            ffi::api().TT_GetUserJitterControl(
                self.ptr.0,
                user_id.raw(),
                ffi::StreamType::STREAMTYPE_VOICE,
                &mut raw,
            ) == 1
        } {
            Some(crate::types::JitterConfig::from(raw))
        } else {
            None
        }
    }

    /// Acquires an audio block for a user.
    pub fn acquire_user_audio_block(
        &self,
        types: u32,
        user_id: UserId,
    ) -> Option<*mut ffi::AudioBlock> {
        unsafe {
            let ptr = ffi::api().TT_AcquireUserAudioBlock(self.ptr.0, types, user_id.raw());
            if ptr.is_null() { None } else { Some(ptr) }
        }
    }

    /// Inserts an audio block into the mixer.
    pub fn insert_audio_block(&self, block: &ffi::AudioBlock) -> bool {
        unsafe { ffi::api().TT_InsertAudioBlock(self.ptr.0, block) == 1 }
    }

    /// Releases a previously acquired audio block.
    ///
    /// # Safety
    /// - `block` must be a pointer returned by `acquire_user_audio_block`.
    /// - The block must not be released more than once.
    /// - The pointer must not be used after release.
    pub unsafe fn release_user_audio_block(&self, block: *mut ffi::AudioBlock) -> bool {
        if block.is_null() {
            return false;
        }
        unsafe { ffi::api().TT_ReleaseUserAudioBlock(self.ptr.0, block) == 1 }
    }

    /// Sets a user's stream volume.
    pub fn set_user_volume(
        &self,
        user_id: UserId,
        stream_type: ffi::StreamType,
        volume: i32,
    ) -> bool {
        unsafe { ffi::api().TT_SetUserVolume(self.ptr.0, user_id.raw(), stream_type, volume) == 1 }
    }

    /// Starts a sound loopback test.
    pub fn start_sound_loopback_test(
        &self,
        in_id: SoundDeviceId,
        out_id: SoundDeviceId,
        rate: i32,
        chans: i32,
        duplex: bool,
    ) -> *mut ffi::TTSoundLoop {
        unsafe {
            ffi::api().TT_StartSoundLoopbackTest(
                in_id.raw(),
                out_id.raw(),
                rate,
                chans,
                if duplex { 1 } else { 0 },
                std::ptr::null(),
            )
        }
    }

    /// Closes a sound loopback test handle.
    ///
    /// # Safety
    /// - `loopback` must be a pointer returned by `start_sound_loopback_test`.
    /// - The handle must not be closed more than once.
    pub unsafe fn close_sound_loopback_test(&self, loopback: *mut ffi::TTSoundLoop) -> bool {
        if loopback.is_null() {
            return false;
        }
        unsafe { ffi::api().TT_CloseSoundLoopbackTest(loopback) == 1 }
    }

    fn subscribe_audio_blocks<S>(
        &self,
        user_id: UserId,
        types: u32,
        sink: S,
    ) -> AudioBlockSubscription<'_>
    where
        S: AudioBlockSink + Send + 'static,
    {
        let sink = Arc::new(Mutex::new(sink));
        let sink_ref = Arc::clone(&sink);
        let subscription_id = self
            .on_event(Event::AudioBlock)
            .filter_user(user_id)
            .subscribe(move |ctx| {
                let client = ctx.client();
                let Some(ptr) = client.acquire_user_audio_block(types, user_id) else {
                    return;
                };
                let block = unsafe { &*ptr };
                if let Some(view) = AudioBlockView::from_block(block)
                    && let Ok(mut sink) = sink_ref.lock()
                {
                    sink.handle(&view);
                }
                unsafe {
                    let _ = client.release_user_audio_block(ptr);
                }
            });

        AudioBlockSubscription {
            client: self,
            subscription_id,
            user_id,
            stream_types: types,
        }
    }
}
