pub fn play_audio() {
    use rodio::{Decoder, OutputStream, Sink};
    use std::io::Cursor;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let audio_data = include_bytes!("../resources/metal-crusher.mp3");
    let cursor = Cursor::new(audio_data);
    let decoder = Decoder::new(cursor).unwrap();
    sink.append(decoder);

    std::thread::spawn(move || {
        sink.sleep_until_end();
    });
}