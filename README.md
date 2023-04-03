# 描きくん - kakikun
[![AUR](https://img.shields.io/aur/version/kakikun?color=green)](https://aur.archlinux.org/packages/kakikun/)

Kakikun is a tool to paint, draw and create ASCII art in your terminal. (The twist is that it's really unicode art.)

![screenshot](https://github.com/file-acomplaint/file-acomplaint/blob/main/assets/screenshot.png?raw=true)

## Features
Kakikun lets you draw to your terminal like a canvas. You can use any character as your brush! Also, you can apply some filters like blur to your background colours. In addition to loading and saving images, you can also store your finished work as text or as a kakikun project. As can see above, kakikun sports a painter's colour picker with hue slider; I can hear the Microsoft Paint devs shiver already!

![screenshot](https://github.com/file-acomplaint/file-acomplaint/blob/main/assets/screenshot2.png?raw=true)

## Installation
The easiest way to try it out is downloading a binary - either for ![Linux](https://github.com/file-acomplaint/kakikun/releases/download/v0.1.0/kakikun.exe) or ![Windows](https://github.com/file-acomplaint/kakikun/releases/download/v0.1.0/kakikun.exe) - and running that in your terminal. If you use Arch - by the way - you are lucky and can install comfortably from the ![AUR](https://aur.archlinux.org/packages/kakikun/).

If you want to build it yourself, you can run:
```bash
git clone https://github.com/file-acomplaint/kakikun.git
cd kakikun
cargo run --release
```

And then you can move target/release/kakikun where you want. If you'd like some other installation medium, tell me and I'll do my best :)

## Heritage
Kakikun is written in Rust and a mischievous child of the TUI (Text User Interface) library ![cursive](https://github.com/gyscos/cursive). Another star of the show is the Rust image processing library. Thanks lots! 

### Compatibility
Kakikun is *very close* to being cross-plattform. I personally can only test for Archlinux and Windows, of which the former does a lot better. If there are any MacOS, Redox or other Unix users in the audience tonight: You have been chosen as a beta tester just now, congrats!

![screenshot](https://github.com/file-acomplaint/file-acomplaint/blob/main/assets/screenshot3.png?raw=true)
