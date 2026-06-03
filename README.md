# ree-lib-rs
A rust library for RE engine stuff.

Provides:

1. some game file format parseing
2. [reframework bindings](./reframework) for making native rust reframework plugins
3. kinda fast i think? il2cpp, rsz and enum dump parsing

## TODO
- [] rsz read (from json)
- [] rsz write (to binary)
- [] more file formats (could use ree-pak-rs for paks)
- [] versioned file formats
- [] better reframework bindings
- [] codegen for rsz types to deserialize from rsz
- [] *MAYBE (unlikely)* add core save file stuff here from [ree-save-editor](https://github.com/kvasszn/ree-save-editor) but idk (like crypto functions and the binary format ser/de)
