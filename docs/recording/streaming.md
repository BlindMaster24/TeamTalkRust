# Streaming audio blocks

Use audio block streaming when you need real-time audio processing, custom
transport, or external recording pipelines.

## Callback sink

```rust
use teamtalk::{CallbackSink};

let sink = CallbackSink(|block| {
    let _ = block.sample_rate;
    let _ = block.channels;
});

let _sub = client.stream_audio_blocks(user_id, stream_types, sink);
```

## UDP sink

```rust
use teamtalk::UdpSink;

let sink = UdpSink::connect("127.0.0.1:9000").ok();
if let Some(sink) = sink {
    let _sub = client.stream_audio_blocks(user_id, stream_types, sink);
}
```

## Writer sink

```rust
use std::fs::File;
use teamtalk::WriterSink;

let file = File::create("audio.raw")?;
let sink = WriterSink::new(file);
let _sub = client.stream_audio_blocks(user_id, stream_types, sink);
```

Notes:
- Raw audio blocks are 16-bit PCM frames.
- Call `client.enable_audio_block_event_ex` if you need a custom audio format.
