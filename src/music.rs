use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{io::Cursor, sync::LazyLock};

struct State {
    stream_handle: OutputStreamHandle,
    sink: Sink,
    cursor: std::io::Cursor<&'static[u8]>
}

impl State {
    fn new() -> Self {
        State {
            stream_handle: create_new_output_stream(),
            sink: create_new_sink(),
            cursor: create_music_cursor()
        }
    }
}

static MUSIC_INSTANCE: LazyLock<State> = std::sync::LazyLock::new(|| State::new());

fn create_new_output_stream() -> OutputStreamHandle {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    return stream_handle;
}

fn create_new_sink() -> Sink {
    let sink = Sink::try_new(&MUSIC_INSTANCE.stream_handle).unwrap();
    return sink;
}

fn create_music_cursor() -> std::io::Cursor<&'static[u8]> {
    let bytes = include_bytes!("../resources/metal-crusher.mp3");
    let cursor: Cursor<&[u8]> = Cursor::new(bytes);

    return cursor;
}

pub fn play_metal_crusher_music() {
    let source = Decoder::new(MUSIC_INSTANCE.cursor.clone()).unwrap();
    MUSIC_INSTANCE.sink.append(source);
}