# A Ray Caster in Rust (+ Webassembly)
This is a small sample project I used for learning Rust and experimenting with Webassembly. It is a simple [Ray Caster][1] that renders a simple 3D maze completely in Software. There is also a [Swift][2] version of the Ray Caster. It also contains a detailed introduction into Ray Casting with code examples.

Try the [browser version][3] to play with it instantly or compile on your desktop!

![][image-1]

## Supported Plattforms
**Browser:**
iOS 11.3+ (iOS 11.2.x is broken…)
Android 8.0
At least: Safari 11, Chrome, Firefox

**Native:**
macOS, Linux, Windows

## How to Build
To build the desktop or the browser version you need to install the Rust  compiler first:

```bash
curl https://sh.rustup.rs -sSf | sh
```

During the installation, you may be asked for installation options. Just press Enter to select the default option. After the installation succeeded make sure that all environment variables are set up:

```bash
source ~/.profile
```

You may also need to install libSDL. On macOS you can do this with brew:

```bash
brew install sdl2
```

On Ubuntu / Debian:

```bash
apt install libsdl2-dev
```

### The Desktop Version
You can build an run the desktop version by typing 

```bash
cargo run
```

### The Browser Version
To build the browser version, you need to install the [Emscripten SDK][4]. Create a new folder on your file system and open it in your terminal. Then run the following commands to install the SDK:

```bash
git clone https://github.com/juj/emsdk.git
cd emsdk
./emsdk update
./emsdk install latest
./emsdk activate latest
source ./emsdk_env.sh
embuilder.py build sdl2
```

We also need to setup Webassembly support for Rust:

```bash
rustup target add wasm32-unknown-emscripten
```

Finally, switch back to the ray caster source folder to build the browser version:

```bash
./build-web.sh
```

The compilation result is then stored to the  folder `html`. Since Webassembly can’t be directly embedded to a HTML page you need a web server for running the binary. Just copy the entire `html` folder to your web server and then open the `index.html` page in your browser.

## Changing the Map
You can change the map by editing the text file in „assets/map.txt“. Each character represents a field in the map. Use the space character for empty fields. You can use „R“, „G“, „B“, „Y“, „O“ to create colored walls. The map is embedded during compilation, so every change of the map requires a recompilation.

## Feedback
I’m still a Rust novice: I’m really interested in any feedback regarding my Rust code.

[1]:	https://wikipedia.org/wiki/Ray_casting
[2]:	https://github.com/hydrixos/raycaster-swift
[3]:	https://cdn.rawgit.com/hydrixos/raycaster-rust/df4db06b/html/index.html
[4]:	https://emscripten.org

[image-1]:	doc/Readme.gif