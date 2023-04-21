# RustNES

## Description
---
RustNES is a NES music creation tool written entirely in Rust. While not entirely faithful to the NES sound, it's a first attempt at a modern recreation.

## Demo
[![Watch the video](https://img.youtube.com/vi/UkP5NZZ0tf8/default.jpg)](https://www.youtube.com/watch?v=UkP5NZZ0tf8)

## How to use RustNES

RustNES is a sequencer that has the ability to make music using 4 channels. 2 pulse waves, 1 triangle, and 1 noise channel. Song can be played, and saved to file using a custom file format .rsf (RustNES Save Format). 

RustNES has 2 main sections; the note sequencer, and the channel selector. 

### The Note Sequencer
The note stepper allows the user to select the pitch of an individual note. A note can be individually left clicked to change one notes pitch, or dragged to select multiple at once. The pitch of a note will be played played upon selection to help song creation. To remove a note, hold down right click on a note.

### The Channel Selector
RustNES allows the user change which channel is selected using the channel selector on the bottom of the window. Each channel has a corresponding symbol for easy understanding. The channels in each column all play in parallel, allowing more advanced music creation. To increase the length of a song, press the large button with a "+". To shorten the song press one of the "-" buttons above a channel. 

## Original Milestones
---
### Milestone 1 (worst case scenario):
>* Basic audio generator that can produce 8-bit sound based on user's keyboard input.

### Milestone 2 (expected):
>* The program can accept MIDI file as input, and convert it to chiptune sound. It should be able to handle multiple channels as well.
Various MIDI files should be used for testing this function, such as a relatively simple classical music file with a single channel, and more complex pop songs with multiple channels. <br /><br />
>* Graphical User Interface is implemented with features such as play/pause, volume control, and MIDI import, export function.<br /><br />
>* Animation of waveforms is displayed as well when music is played (something like old Windows' winamp visualization)
We could test this function by comparing the visualization with other pre-existing services.

### Milestone 3 (advanced):
>* An equalizer is added and users can use it to modify the sound, like base boost, treble boost, etc.
This also can be tested by utilizing other services, such as spotify's equalizer function and comparing the difference 
in quality.

## Milestone Postmortem
The original milestone were set early in the project timeline. As people realised that other courses were taking most of their time, there was less work being added to the main repository. This repository can be seen as a hard fork of the original idea. As it stands, this version of the project was coded entirely by myself, and has significantly less features in regards to importing and exporting than originally planned, as those were less important than the core functionality.

This project will most likely continue as a small side project, and eventually include both MIDI and NSF support. It would be interesting to eventually included added functionality such as the waveform viewer, an equalizer, and potententially samples. However, the project will need a sizable rewrite to support these features, as it currently contains many loose ends.

## What was Learned
- The NES is a fairly terrible device for creating music. With its non-linear mixing and strange filters choise, it fails to be consistent in how sounds will play under a variety of conditions. This is what makes it interesting.

- MIDI has many edge cases. As an example, to start playing a MIDI note, there is a note on flag, but to turn a note off, there are two(maybe more) ways. 
1. Use a note off flag.
2. Use a note on with a velocity of 0.

While this doesn't seem particularly difficult to add, issues like this compound in annoying ways.

- Egui is significantly more performant that originally thought. As RustNES uses egui, which is an immediate mode GUI crate, there was a concern that the amount of note pitches buttons needed would bottleneck the allowed notes on screen. This never happened. Even with 32 pitches per note, and 32 notes per page it still ran well. For clarity however, the number was lowered back to 24 pitches.

- While rust is complicated at first, it is an extremely intersting language.

## Contributors
---
[Cy Chung](https://github.com/crschung), [Jae Park](https://github.com/jpark052), [Spencer Hart](https://github.com/Syyreign)

## Contributions

### Cy Chung
- README/Documentation

### Jae Park
- README/Documentation

### Spencer Hart
- Create base code
- Waves and Oscillators
- Filters (high and low pass)
- NES non-linear Mixer
- File saving and loading using the custom extension `.rsf` (RustNES Sound Format)
- The GUI
<img width="300" alt="NES_GUI" src="https://user-images.githubusercontent.com/7028156/233228911-59ea7c7f-a47a-4d90-84c4-fba33d0df45b.png">

## Resources
---
>* [Retro Game Mechanics Explained Playlist](https://www.youtube.com/playlist?list=PLHQ0utQyFw5JD2wWda50J8XuzQ2cFr8RX)
>* https://www.youtube.com/watch?v=8RrQrATnXXY
>* https://www.egui.rs/
>* [NES Basics](https://bugzmanov.github.io/nes_ebook/)
>* [Crate Midly](https://docs.rs/midly/latest/midly/)
>* https://docs.rs/midly/latest/rodio
>* https://github.com/RustAudio/rodio
>* https://docs.rs/basic_waves/latest/basic_waves/index.html
>* https://www.youtube.com/watch?v=gKXGDuKrCfA
>* https://www.nesdev.org/wiki/Nesdev_Wiki

## Relevant Research Papers
---
>* [Automatic conversion of pop music into chiptunes for 8-bit pixel art](https://ieeexplore.ieee.org/abstract/document/7952188)
>* [Music Genre Classification Using MIDI and Audio Features](https://link.springer.com/content/pdf/10.1155/2007/36409.pdf)
>* [Melody extraction on MIDI music files](https://ieeexplore.ieee.org/abstract/document/1565863/)

## Programming Language
---
Rust

## How To Run
---
1. Install Rust: https://www.rust-lang.org/tools/install
2. Navigate to the root of the project and type:
```
cargo build
cargo run
```

