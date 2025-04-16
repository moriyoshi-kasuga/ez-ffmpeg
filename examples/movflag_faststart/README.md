# ez-ffmpeg Example: Movflag Faststart

## Equivalent FFmpeg Command

```sh
ffmpeg -i test.mp4 -c copy -movflags faststart output.mp4
```

## Code Explanation

- **Input Video:** `test.mp4`
- **Faststart Enabled:** The `movflags=faststart` flag moves the MP4 metadata (`moov atom`) to the beginning of the file, allowing the video to start playing before it has fully downloaded.
- **Stream Copy:** The `-c copy` option ensures that no re-encoding occurs, making the process fast and preserving the original quality.
- **Output Video:** The result is saved to `output.mp4`.

## When to Use

- **Use this method** when you want to enable quick playback start, especially for videos intended for web streaming, without sacrificing quality or processing time.
