use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;


pub fn play_audio() {
    std::thread::spawn(move || {

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
    
        let audio_data = include_bytes!("../resources/metal-crusher.mp3");
        let cursor = Cursor::new(audio_data);
        let decoder = Decoder::new(cursor).unwrap();
        sink.append(decoder);

        sink.sleep_until_end();
    });
}