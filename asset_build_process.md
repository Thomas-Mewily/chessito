# Asset Build Process ⚡

The build process for loading asset into the game is based on macro `macro_asset_loader::import_asset`

```rust
macro_asset_loader::import_asset!("./board_graphic/assets", "./board_graphic/src/generated/generated_assets.rs");
```

The crate `macro_asset_loader` is designed in a flexible way to handle multiple format : 

- `png` for `Texture2D` (gpu) (cpu equivalent is `Image`),
- `wav` for `Sound`,
- `ttf` for `Font`,

Adding extra data is also possible with adding extra extension before the final extension : those are called tag.

For exemple, it is possible to add a tag into a `.png` file to tell if the content is some pixel art of some hd drawing that need anti aliasing.

## The Tag System 🏷️

Tag are defined for one kind of media.

For instance, the `pa` (Pixel Art) tag can only be apply to `Texture2D` (png and other supported format)

The supported format extension can be expended in `macro_asset_loader/src/generate_asset.rs`


When a file is exported, the tag and extension of a file are not part of the asset name.

So `test.px.png` and `test.aa.jpg` will have the same name (thus creating an error)

## What about Folder 📂

#### Named Folder

When applied on a folder, tags are recursive and will apply on any sub file and folder (that suport them).


Writting `.sfx` for each sound is tedious : 👎
```rust
sound_effect
|- a.sfx.wav
|- b.sfx.wav
|- c.sfx.wav
|- ...
|- credit.png
```

This is equivalent and better 👍
```rust
sound_effect.sfx
|- a.wav
|- b.wav
|- c.wav
|- ...
|- credit.png
```
Note that the `credit.png` will **not** receive the tag `.sfx` since the tag is only defined for `Sound`

#### Nameless tagged Folder

Empty folder are special : All of their content will be unpacked into the parent folder.

This is okay :
```
sound
|- a.sfx.wav
|- b.sfx.wav
|- c.sfx.wav
| 
|- m1.music.wav
|- m2.music.wav
|- m3.music.wav
```
But this is equivalent and better : 🔥
```
sound
|- .sfx
|    |- a.wav
|    |- b.wav
|    |- c.wav
|
|- .music
     |- m1.wav
     |- m2.wav
     |- m3.wav
```

## Tag Sheet List

#### Texture2D 🖼️

Anti Aliasing :

- 🟦 `pa` : for Pixel Art, set anti Aliasing of off
- 🖼️ `aa` : set Anti Aliasing on

- ✂️ `px(xy)` / `px((x,y))` : define the size of each tile on the spritesheet
- 📏 `margin(xy)` / `margin((x,y))` : define the margin of each tile on the spritesheet

#### Sound 🔊

- 🎺 `sfx` : define the sound as a Sound Effect
- 🎵 `music` : define the sound as a Music

#### Universal :

- `credits(Your Name Or Some Reference)` To thank to person who make the asset 
- `ignore` Ignore the asset


## Example :



ex : 
- `my_pixel_art.px(32).pa.png` define a 32x32 px tile sized pixel art.
- `my_pixel_art.aa.px((128, 128)).margin(1).png` define a 128x128 px tile with 1 of margin on each side.

-  `bonk.sfx.wav` define a sound effect

The tags will be applied from left to right, but in any case, the ordering don't matter with current tags.

# Author
- Thomas Mewily 2024