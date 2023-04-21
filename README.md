# RustNES (IN_PROGRESS)

## Description
---
Retro chiptune music is starting to make a resurgence with the popularity of many retro indie games like Hades and Celeste. One of the reasons why it's gaining its popularity again is because it resonates with a lot of people; it brings back nostalgia and fond memory associated with it. However, a majority of music produced for these games rarely use actual hardware that made the 8-bit and 16-bit music charming. The purpose of this project is to replicate a sound chip from the NES (Nintendo Entertainment System) using Rust, and bring back those memories to the users.

## Demo
[![Watch the video](https://img.youtube.com/vi/UkP5NZZ0tf8/default.jpg)](https://www.youtube.com/watch?v=UkP5NZZ0tf8)

## Milestones
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

## Contributors
---
[Cy Chung](https://github.com/crschung), [Jae Park](https://github.com/jpark052), [Spencer Hart](https://github.com/Syyreign), [David Kim](https://github.com/Quayvid), [Ethan Slogotski](https://github.com/eman1003), [Francis German](francisgerman70), [Skylar Buck](https://github.com/Skylar777)

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
Mainly Rust, and possibly Python as well

## Expected Functions:
---
* MIDI Input
* Visualization (waveform)
* GUI
* Pass songs using import 

## Potential Functions:
---
* Filters?
* Entrie chip (real code)
* Export and Save (.wav, .ogg, .mp3)

## Dependencies
---
* [Rodio](https://github.com/RustAudio/rodio)
* [egui](https://github.com/emilk/egui)


## How To Run
---
1. Install Rust: https://www.rust-lang.org/tools/install
2. Navigate to the root of the project and type:
```
cargo build
cargo run
```

