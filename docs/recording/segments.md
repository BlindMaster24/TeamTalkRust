# Segment naming and rotation

`RecordingOptions::template` controls segment file names.

## Template rules

- If the template contains `{index}`, it is replaced with the segment number.
- Otherwise, the file name becomes `name.partN.ext`.

Examples:

```
recordings/session-{index}.ogg  -> recordings/session-1.ogg
recordings/session.ogg          -> recordings/session.part1.ogg
```

## Rotation strategy

Use `segment()` or `handle_event()` when you detect:
- channel changes,
- codec changes,
- time-based rotation (e.g., hourly),
- size-based rotation.

Use `rotate_if_needed()` when you configure `max_duration` or `max_size_bytes`.
Use `handle_event(event, message)` to rotate automatically on channel or codec
changes without manual checks.
Use `start_current_channel()` when you want the session to follow your joined
channel and segment on channel changes.
