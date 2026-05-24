# quickserve
An (admittedly hastily written) executable that allows you to serve a folder over the
network with no hassle over HTTP. Just `./quickserve -d <FOLDER HERE>`. No fuss no muss 
just bang and done. That's it. You barely even have to think about it once you remember it.

Also secretly reference documentation for myself in case I forget how to build an Actix
app again - if I even bother to refer back to it...

---
### Platforms
Tested on Darwin (macOS 26.5) at the time of writing. `quickserve` is probably simple enough
to build and work on any POSIX platform or Windows with no issues provided that your environment
has a somewhat supported Rust toolchain and that can build [Actix](https://actix.rs). Haven't
tested it on anything else other than Darwin though.

TLDR: *I've only tried it on macOS 26.5. I don't guarantee that `quickserve` works on other 
platforms or on future macOS releases.*

---
### Building
If you have a Rust toolchain installed, it should be as simple as `cargo build --release`
and the result should be in `target/release/quickserve` unless you redirected your target
folder.

---
### Arguments
```
Usage: quickserve [OPTIONS] --directory <DIRECTORY>

Options:
-d, --directory <DIRECTORY>  Target directory
-p, --port <PORT>            Port to listen on
-i, --ip <IP>                IP address to listen on
```

---
### License
[Unlicense. Public domain work.](LICENSE) Use anything in this repo as you wish - however please keep in mind that
dependencies are still under their [original licenses.](THIRDPARTYLICENSES.md)

You are responsible for complying with the terms of those individual licenses when distributing the software.