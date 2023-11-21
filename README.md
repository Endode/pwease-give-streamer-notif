# Pwease Give Streamer Notif

### About
Runs in the background and sends a notification when a streamer is streaming

Supports YouTube and Twitch

Note: Twitch itself can be slow to update the stream thumbnail preview, which is what this program uses to determine whether a streamer is live or not. So it may take a few minutes to update

### Usage
## FREQUENCY IS NOT IMPLEMENTED YET
```console
$ pwease-give-streamer-notif <username> <platform> <username2> <platform2> [ <frequency> ]
```
#### Examples
## FREQUENCY IS NOT IMPLEMENTED YET
Checks to see if "tsoding" is streaming on Twitch every 10 minutes
```console
$ pwease-give-streamer-notif tsoding twitch 10m
```
Checks to see if "coolstreamer123123" is streaming on YouTube every hour
```console
$ pwease-give-streamer-notif coolstreamer123123 youtube 1h
```
Checks to see if "x_programmergal_x" is streaming on YouTube using the default frequency
```console
$ pwease-give-streamer-notif x_programmergal_x youtube
```

### Building
Rust and Cargo are required

The following command builds the project in release mode
```console
$ cargo build -r
```
