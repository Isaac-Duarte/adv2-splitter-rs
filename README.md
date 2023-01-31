# adv2-splitter-rs
Advanced Duplicator 2 dupe file splitter. A small utility written in rust to deserialize and split saved duplication files into many.

# Why?
Simple problems need complicated solutions

### ok... What was the problem?
I created too big of a dupe and wanted to split amongst friends due to a prop limit

# Codec Support
Only the latest version 5 of the codec is supported for now, and most likely will be the only supported codec.

You can look at the [Lua implementation](https://github.com/wiremod/advdupe2) to see different codec versions


# Documentation
All the documentation is in the code itself as well as examples. To understand the codec implementation please view the 
in depth specification in the `codec.rs` on the `AdCodec5` struct.