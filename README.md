# Pwease Give Streamer Notif

### About
Runs in the background and sends a notification when a streamer is streaming

Supports YouTube and Twitch

### Usage
```console
$ pwease-give-streamer-notif <username> <platform> [ <frequency> ]
```
#### Examples
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
Rust and Cargo is required

The following command builds the project in release mode
```console
$ cargo build -r
```
