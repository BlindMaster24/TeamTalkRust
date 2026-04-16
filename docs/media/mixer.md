# Mixer Control

Control Windows audio mixer devices: volume, mute, boost, and input selection.

**Platform:** Windows only. All methods are gated by `#[cfg(windows)]` and are unavailable on other platforms.

## Device Enumeration

- `get_mixer_count()` — number of mixer devices
- `get_mixer_name(index)` — device name by index
- `get_mixer_wave_in_name(wave_id)` — input device name
- `get_mixer_wave_out_name(wave_id)` — output device name

```rust
for i in 0..client.get_mixer_count() {
    println!("Mixer {}: {}", i, client.get_mixer_name(i));
}
```

## Output Control

Control playback volume and mute state for output devices:

- `set_mixer_output_mute(wave_id, control, mute)` — mute or unmute
- `get_mixer_output_mute(wave_id, control)` — returns mute state (`i32`)
- `set_mixer_output_volume(wave_id, control, vol)` — set volume level
- `get_mixer_output_volume(wave_id, control)` — returns volume level (`i32`)

`control` is a `ffi::MixerControl` value that selects the specific mixer control (e.g. master, wave, microphone).

## Input Control

Control recording volume, mute, and boost for input devices:

- `set_mixer_input_mute(wave_id, mute)` — mute or unmute input
- `get_mixer_input_mute(wave_id)` — returns input mute state (`i32`)
- `set_mixer_input_boost(wave_id, enable)` — enable or disable input boost
- `get_mixer_input_boost(wave_id)` — returns boost state (`i32`)

## Input Selection

Enumerate and select input controls (e.g. microphone, line-in):

- `get_mixer_input_control_count(wave_id)` — number of input controls
- `get_mixer_input_control_name(wave_id, index)` — control name
- `set_mixer_input_control_selected(wave_id, index)` — select an input control
- `get_mixer_input_control_selected(wave_id, index)` — whether a control is selected

For enum-style input controls:

- `set_mixer_input_selected(wave_id, control)` — select a mixer input by `MixerControl`
- `get_mixer_input_selected(wave_id, control)` — returns selected state (`i32`)
- `set_mixer_input_volume(wave_id, control, vol)` — set input volume
- `get_mixer_input_volume(wave_id, control)` — returns input volume (`i32`)

## Example

```rust
// List input controls for the first wave device
let count = client.get_mixer_input_control_count(0);
for i in 0..count {
    let name = client.get_mixer_input_control_name(0, i);
    let selected = client.get_mixer_input_control_selected(0, i);
    println!("  Control {}: {} (selected={})", i, name, selected);
}

// Mute output
client.set_mixer_output_mute(0, MixerControl::Master, true);
```
