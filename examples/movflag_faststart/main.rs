use ez_ffmpeg::FfmpegContext;
use ez_ffmpeg::Output;

fn main() {
    FfmpegContext::builder()
        // Specify the input file. Stream index is 0.
        .input("test.mp4")
        .output(
            Output::new("output.mp4")
                // Copy the video stream (v) from input stream #0
                .add_stream_map_with_copy("0:v")
                // Copy the audio stream (a) from input stream #0
                .add_stream_map_with_copy("0:a")
                // Move the 'moov' atom to the beginning of the file
                // for faster streaming playback
                .set_format_opt("movflags", "faststart"),
        )
        .build()
        .unwrap()
        .start()
        .unwrap()
        .wait()
        .unwrap();
}
