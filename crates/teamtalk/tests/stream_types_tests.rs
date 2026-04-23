//! Integration tests for the `StreamTypes` newtype.
//!
//! The newtype wraps a `u32` bitmask and must:
//!
//! * Round-trip through `from_raw` / `raw` for any input pattern.
//! * Expose the same constants as the underlying
//!   `teamtalk_sys::StreamType::STREAMTYPE_*` enum so callers can
//!   migrate without behaviour changes.
//! * Support the usual bitwise combinators (`|`, `|=`, `&`, `&=`,
//!   `!`) and `contains_*` helpers.
//! * Convert to / from `u32` through `From` so existing callers can
//!   pass either a raw bitmask or a typed value to APIs that take
//!   `impl Into<StreamTypes>`.

use teamtalk::types::StreamTypes;
use teamtalk_sys as ffi;

#[test]
fn constants_match_ffi_stream_type_values() {
    assert_eq!(
        StreamTypes::NONE.raw(),
        ffi::StreamType::STREAMTYPE_NONE as u32
    );
    assert_eq!(
        StreamTypes::VOICE.raw(),
        ffi::StreamType::STREAMTYPE_VOICE as u32
    );
    assert_eq!(
        StreamTypes::VIDEO_CAPTURE.raw(),
        ffi::StreamType::STREAMTYPE_VIDEOCAPTURE as u32
    );
    assert_eq!(
        StreamTypes::MEDIAFILE_AUDIO.raw(),
        ffi::StreamType::STREAMTYPE_MEDIAFILE_AUDIO as u32
    );
    assert_eq!(
        StreamTypes::MEDIAFILE_VIDEO.raw(),
        ffi::StreamType::STREAMTYPE_MEDIAFILE_VIDEO as u32
    );
    assert_eq!(
        StreamTypes::DESKTOP.raw(),
        ffi::StreamType::STREAMTYPE_DESKTOP as u32
    );
    assert_eq!(
        StreamTypes::DESKTOP_INPUT.raw(),
        ffi::StreamType::STREAMTYPE_DESKTOPINPUT as u32
    );
    assert_eq!(
        StreamTypes::MEDIAFILE.raw(),
        ffi::StreamType::STREAMTYPE_MEDIAFILE as u32
    );
    assert_eq!(
        StreamTypes::CHANNEL_MSG.raw(),
        ffi::StreamType::STREAMTYPE_CHANNELMSG as u32
    );
    assert_eq!(
        StreamTypes::LOCAL_MEDIAPLAYBACK_AUDIO.raw(),
        ffi::StreamType::STREAMTYPE_LOCALMEDIAPLAYBACK_AUDIO as u32
    );
    assert_eq!(
        StreamTypes::CLASSROOM_ALL.raw(),
        ffi::StreamType::STREAMTYPE_CLASSROOM_ALL as u32
    );
}

#[test]
fn empty_is_zero_and_default() {
    assert_eq!(StreamTypes::empty().raw(), 0);
    assert!(StreamTypes::empty().is_empty());
    assert_eq!(StreamTypes::default(), StreamTypes::empty());
    assert!(!StreamTypes::VOICE.is_empty());
}

#[test]
fn from_raw_round_trips_arbitrary_bits() {
    // The newtype must preserve unknown bits so callers reading a
    // bitmask from an event do not silently lose information.
    let weird = 0xABCD_1234_u32;
    let wrapped = StreamTypes::from_raw(weird);
    assert_eq!(wrapped.raw(), weird);
    let u: u32 = wrapped.into();
    assert_eq!(u, weird);
    let back: StreamTypes = weird.into();
    assert_eq!(back, wrapped);
}

#[test]
fn bitor_combines_and_is_commutative() {
    let a = StreamTypes::VOICE | StreamTypes::MEDIAFILE_AUDIO;
    let b = StreamTypes::MEDIAFILE_AUDIO | StreamTypes::VOICE;
    assert_eq!(a, b);
    assert_eq!(a.raw(), 1 | 4);
    assert!(a.contains_all(StreamTypes::VOICE));
    assert!(a.contains_all(StreamTypes::MEDIAFILE_AUDIO));
    assert!(!a.contains_any(StreamTypes::DESKTOP));
}

#[test]
fn bitor_assign_updates_in_place() {
    let mut mask = StreamTypes::VOICE;
    mask |= StreamTypes::DESKTOP;
    assert_eq!(mask, StreamTypes::VOICE | StreamTypes::DESKTOP);
    assert!(mask.contains_all(StreamTypes::VOICE));
    assert!(mask.contains_all(StreamTypes::DESKTOP));
}

#[test]
fn bitand_and_not_mask_correctly() {
    let both = StreamTypes::VOICE | StreamTypes::DESKTOP;
    let only_voice = both & StreamTypes::VOICE;
    assert_eq!(only_voice, StreamTypes::VOICE);

    let mut mask = both;
    mask &= StreamTypes::DESKTOP;
    assert_eq!(mask, StreamTypes::DESKTOP);

    // Dropping voice from CLASSROOM_ALL should leave exactly the
    // other bits (media + desktop + desktop_input = 94).
    let classroom_without_voice = StreamTypes::CLASSROOM_ALL & !StreamTypes::VOICE;
    assert_eq!(classroom_without_voice.raw(), 95 & !1);
}

#[test]
fn contains_any_vs_all() {
    let mask = StreamTypes::VOICE | StreamTypes::DESKTOP;
    let voice_or_video = StreamTypes::VOICE | StreamTypes::VIDEO_CAPTURE;
    assert!(mask.contains_any(voice_or_video));
    assert!(!mask.contains_all(voice_or_video));
    assert!(mask.contains_all(StreamTypes::VOICE));
    assert!(mask.contains_all(StreamTypes::DESKTOP));
}

#[test]
fn u32_and_stream_types_are_interchangeable_via_into() {
    // The public recording APIs accept `impl Into<StreamTypes>`;
    // verify both a raw u32 and a typed StreamTypes round-trip to
    // the same wrapped value.
    fn accept(types: impl Into<StreamTypes>) -> StreamTypes {
        types.into()
    }
    assert_eq!(accept(1u32), StreamTypes::VOICE);
    assert_eq!(accept(StreamTypes::VOICE), StreamTypes::VOICE);
    assert_eq!(
        accept(StreamTypes::VOICE | StreamTypes::DESKTOP).raw(),
        1 | 16,
    );
}

#[test]
fn classroom_all_matches_documented_value() {
    // The SDK documents CLASSROOM_ALL as 95. Bit composition:
    //   voice(1) + video_capture(2) + mediafile_audio(4) +
    //   mediafile_video(8) + desktop(16) + channel_msg(64) = 95.
    // Note that desktop_input(32) is NOT included; CHANNEL_MSG is.
    assert_eq!(StreamTypes::CLASSROOM_ALL.raw(), 95);
    assert!(StreamTypes::CLASSROOM_ALL.contains_all(StreamTypes::VOICE));
    assert!(StreamTypes::CLASSROOM_ALL.contains_all(StreamTypes::VIDEO_CAPTURE));
    assert!(StreamTypes::CLASSROOM_ALL.contains_all(StreamTypes::MEDIAFILE));
    assert!(StreamTypes::CLASSROOM_ALL.contains_all(StreamTypes::DESKTOP));
    assert!(StreamTypes::CLASSROOM_ALL.contains_all(StreamTypes::CHANNEL_MSG));
    assert!(!StreamTypes::CLASSROOM_ALL.contains_any(StreamTypes::DESKTOP_INPUT));
}
