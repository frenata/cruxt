# cruxt

Tool(s?) for manipulating recorded fitness files, ala TCX/GPX/etc.

## Project Goals

  * build good library support for de/serializing TCX files (the existing library in Rust doesn't support serialization and has little traffic)
  * build a binary that can collect multiple files and *merge* them, producing a new file that contains all the information that the individual files do
    * this is pretty common for endurance athletes, where multiple devices record different kinds of data, so you want to aggregate!
    * in particular, this should have sane defaults but allow the user to specify which type of data to take from which file

## Personal Goals

  * do something interesting with Rust
  * deattach from my personal over-reliance on non-free software that does a similar merging task
  * explore serverless deployment

## (probably) Non Goals

  * a bunch of integration work with various software providers to fetch/submit TCX files, I just want to focus on file manipulation, not complicated auth work
  * 'weird' manipulation of the recordings beyond merging
  * too many formats of input/output files

## TODO v0.1.0 (aka works-for-my-usecase)

  * wait/work on the sequencing fix as listed here: https://github.com/RReverser/serde-xml-rs/pull/187
  * wait/work on attribute/tag fix, or just move to yaserde! https://github.com/media-io/yaserde
  * actually figure out how to merge trackpoints
  * figure out how to selectively drop data streams
    * position
    * altitude
    * distance
    * cadence
    * HR
    * Watts
  * compare parity with other tools
