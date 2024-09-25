<!-- @format -->

# File management and Streaming system

Built to manage videos and stream them from local file system on demand.

It integrates with VLC to stream media directly to VLC on any device connected to the same network.
This allows user to enable streaming of subtitles (if any) and various Playback sounds (if any), streaming of `.mkv` files with embedded subtitles and audio tracks for example.

## Crates used
Refer to `Cargo.toml` for current dependencie and their versions.

## Project Structure

### Api Module

Making use of `axum` crate, this module takes care of sending data to the api endpoint.

### Disk Module

Utilising tokio's async file system, this module provide api controllers with byte streams of selected file or list of files as per demand.

## End Points 

### `/file?path=`
Specify the path to get a byte stream of data. Currently hardcoded to `.mkv/.mp4`.

### `/files?path=`
Get a list of all files/folder in the specified path.

> Note
> All paths are relative to the path of homelab.exe file.

## Plans

1. Streaming of video files (.mp4,.mkv) over HTTP 1.1 ☑️
2. Streaming video files with subtitles. ☑️
2. Integration with VLC. (partial)
3. Ability to download file from an external URL
4. Changing protocol from HTTP 1.1 to a more efficient one (DASH/HTTP2/HLS).

If you got any idea, improvements or a bug report, feel free to report in `Issues` tab.
If you want to contribute you can always contact me ;)