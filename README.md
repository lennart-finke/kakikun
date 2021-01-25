# 描きくん - kakikun

Kakikun is a tool to paint, draw and create ASCII art in your terminal. (The twist is that it's really unicode art.)

![screenshot](https://github.com/file-acomplaint/file-acomplaint/blob/main/assets/screenshot.png?raw=true)

## Features
Kakikun lets you draw to your terminal like a canvas. You can use any character as your brush! Also, you can apply some filters like blur to your background colours. In addition to loading and saving images, you can also store your finished work as text or as a kakikun project. As can see above, kakikun sports a painter's colour picker with hue slider; I can hear the Microsoft Paint devs shiver already!

![screenshot](https://github.com/file-acomplaint/file-acomplaint/blob/main/assets/screenshot2.png?raw=true)
### Heritage
Kakikun is written in Rust and a mischievous child of the TUI (Text User Interface) library ![cursive](https://github.com/gyscos/cursive). Another star of the show is the Rust image processing library. Thanks lots! 

## Compatibility
Kakikun is *close* to being cross-plattform. The best experience is currently had on UNIX systems, but it does run on Windows as well. If there are any MacOS or Redox users in the audience tonight: You have been chosen as a beta tester just now, congrats!
The issue comes down to finding a ![cursive backend](https://github.com/gyscos/cursive/wiki/Backends) that supports kakikun's features and runs OK on your system, which shouldn't be that difficult. The UNIX and Windows versions use termion and crossterm backend respectively.

![screenshot](https://github.com/file-acomplaint/file-acomplaint/blob/main/assets/screenshot3.png?raw=true)

## The Drawback
I'm not that great at software development and the project has really been a challange for me. This is the first piece of code I have actually documented - I would be very happy if you play around with it. I'm guessing it's not that difficult - for someone who knows what they're doing - to make big improvements upon this. So, if the issues up there inspire enough pity in you... ;^)
