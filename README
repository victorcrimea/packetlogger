# Simple pcap-based packet logger


## Build instructions

On linux libpcap has to installed, e.g.
`sudo apt install libpcap-dev`

On MacOS libpcap should be available on the system already.

On Windows [NPcap](https://npcap.com/#download) has to be installed.

Build with `cargo build --release`


## Monitoring

List available devices with
```
sudo ./target/release/packetlogger --list
```

Run monitoring
```
sudo ./target/release/packetlogger [DEVICE] --port 9000
```
If port paramenter not specified port 8000 is used by default. If device is not specified pcp-internal catch-all device `any` is used.


## Hints

To run simplest HTTP server to monitor run:
```
python3 -m http.server [PORT]
```
This will serve files over HTTP from current folder.

To send valid tcp-stream (not actually HTTP, but monitor doesn't care)
```
nc [HOST] [PORT] < file.txt
```
Where file.txt can have any content you wish to send. This will establish a tcp connection on specified port and send file as-is. Should be detected as invalid HTTP request by server, unless file.txt contains valid HTTP dump.
