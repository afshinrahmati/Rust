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

# What is Rust ?

it is Memory safetly and we do not have bug like in C.
speed and Growing Fast because doesnot have runtime and gorbage collection
if you want to change a variable you should define mut that mean we wnat to chnage it in js var or let

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

# Type int

Binary<10>=> 0 _ 2 ^0 + 1 _ 2^1 => 2
. if our number was + , - ==> identify with 0(-),1(+) and all the number are + and we can not store the sign(+) so we can store 8 bit ==> 256
but if we want to store sign(-) we have 1 bit for sign and 7 bit ==> 128
8bit withSign(i8) withoutSign(u8)
16bit withSign(i16) withoutSign(u16)
32bit withSign(i32) withoutSign(u32)
64bit withSign(i64) withoutSign(u64)
our system bit withSign(Isize) withoutSign()

# Type Boolean:

let yes:bool =true |false

# type character:

- you can use there utf8;
  let a :char = 'a';

# type Array:

let arr: [u8;10] = [1,2,3,4,....,9];
arr[0] = 90; arr shhould be mut;
when we write an array length 5 we should full all the [] with 0 at first or any number.
if did not want that ==> let b = [0u128;100]
print all value there ==> {:#?},arr;

# type tuple:

it is look like array and big differnet you can store in there diffrien type'
let t:(u8,bool,&str) = (10,true,"afshin")
println!=> t.0
let (xy,z) = t;

# loop:

1. loop ==>
   loop {
   println!("afsj);
   break
   }
   let mut counter:i32 = 0;
   let a = loop {
   if counter > 10 {
   break counter;
   }
   counter += 1;
   }
   println(a:{},a)
2. while
   while counter < 10 {
   println("counter:{}",counter)
   }

3. for i in 0..11 {
   println("i:{}",arr[i])
   }
   for i in arr.iter() [
   println(i)
   ]
   for counter in range(10) {

   }

# function

1.fn main() {} 2. fn example(n:i16){} 3. fn mull(number:u8) -> u8 {
return 2\*8;
} 4. fn factorial(n:u64)->u64{
if n =< 1 {
1
}
return n\* factorial(n-1);
}

# what is expression and statement ?

1.expression some operation had return any thing like 2\*8;
2.statement we do not have retutn any thind let a = 19;

# Garbage collector ?

each time it's come and see which part of in ram don't use and clean and free this.
*for garbage collection we can call each 2 minut that garbage collection come and do this => it happend slow speed in our program.
*for garbage collection we can handle it we code==> it can be mistake

- each variable has one ownership
  RUST end of the scope run the method drop();
  let a = String::from("Hi");
  let b = a; ==> in rust come and move the all data in a to b so it inner call drop(a);
  println!(a) ===> it give to us an error.

fn main() {
let a = String::from("hello");
some(a);
prinln!(a) ==> an error because the ownership belong to some
}

fn some(s:String){
println(input)
}

for handle this error you can use the
a =b;
b=a;
or with borrowing and handling.

# stack:https://www.cs.usfca.edu/~galles/visualization/StackArray.html

*it is on the Ram;
*specified for size; int Array[8];
first in last out.
stack is so fast But if we don't have any information about size of a variable we can not push in stack. so heap help to us to this.

in stack ==> prt(pointer for find the valuable);len(len the value);capacity(how many len save on memmory)

for garbagecollected ==> cpp(delete);rust(drop())

# heap:

*it is on the Ram;
*specified for don't size; int Array[?];

new obj(....)set on heap

type String save on the heap directly;because we don't have any size about it and it may add or less the word there;
let mut my_string = String::from("یک رشته جدید میباشد");
my_string.push_string("ادامه متن")
in Heap ==>index(0)ی index(1) ک
