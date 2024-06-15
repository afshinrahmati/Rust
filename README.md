# Rust

learning Rust

# install Rust

1. https://www.rust-lang.org/tools/install
   1.1) export PATH="$HOME/.cargo/bin:$PATH"
   1.2) source "$HOME/.cargo/env"
2. cargo new My-Project
3. cargo build ==> when you clone from git ...
4. cargo run
   OR
   4.1) cd src/ ==> rustc main.rs -o main.o ==> ./main.o<binary>

## What is Mutable and Immutable :

vaiable can chnage it be ==> Mutable !== Im

# What is Rust ?

it is Memory safetly and we do not have bug like in C.
speed and Growing Fast because doesnot have runtime and gorbage collection

# Cargo :

it is package manger like pip , npm

# Modularity :

can write module;

# Mutable and Immutable:

Mutable: you cna change the value of variable.
Immutable : you can not chnage the vlue of variable.

# Date Racing:

two or more threads in a single process access the same memory location concurrently, and. at least one of the accesses is for writing.
and threads Two chnage the vlaue of the threads One.
const is Immutable and when it call Do not happend Date Racing in our programing.
let mut x = 10; ==> x can change in other line

# variable:

let x = 10; ==> x can not change in other line
let mut x = 10; ==> x can change in other line,mut ==> Mutable
const : type
const can be in globla and write outside our func main;
can not ==> const a = a() it is wrong

Binary<10>=> 0 _ 2 ^0 + 1 _ 2^1 => 2
. if our number was + , - ==> identify with 0(-),1(+) and all the number are + and we can not store the sign(+) so we can store 8 bit ==> 256
but if we want to store sign(-) we have 1 bit for sign and 7 bit ==> 128
8bit withSign(i8) withoutSign(u8)
16bit withSign(i16) withoutSign(u16) 
32bit withSign(i32) withoutSign(u32)
64bit withSign(i64) withoutSign(u64)
our system bit withSign(Isize) withoutSign()
