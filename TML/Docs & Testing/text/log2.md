<260627



So I accidentally forgot to save the log that existed before I started writing today but it was mostly about me finding out about the

different alternatives that existed for gathering process data and then narrowing it down to ETW instead of like WMI or

CreateToolhelp32Snapshot because WMI unnecessarily "wrapped up" and the latter is unoptimal for continously tracking processes. ETW

on the other hand is basically exactly what I was looking for, a low-level system-intergrated API which runs continously and returns

lots of rich data.



And I chose Rust for the language because I wanted to.



260627



Time to start coding and in Rust which I haven't even Hello World'd before.



So it seems that there are other languages that come with ready "ETW Intergration". Here they are:



C#, F# and Visual Basic have deep windows intergration via .NET

And .NET is a cross-platform software development platform that provides libraries, tools and a runtime for a few different languages

whereas C# is the most prominent one. And "cross-platform" refers to the fact that it works on both Linux, Windows, iOS and More.

In principle, .NET works as an abstraction layer between high-level code and different operating systems so that code can stay simple

and system interactions sort of clean.



Of course, abstraction isn't always good because it could scrape away some features that I might like.



C doesn't really have something "in it" that allows for easier interaction with ETW but the windows coding environment is made to be

interacted with using C.



There aren't many setbacks to using C to access ETW.



On the other hand, Rust and every other language generally use the same method accessing ETW. This method is called FFI (Foreign

Function Interface) and it does what it sounds like; makes it possible to call other language (usually C) functions from the native

language.



FFI proves to be one of the most important concepts when doing systems programming in Rust. At its' core it's just Rust calling code

that wasn't written in Rust, but how does it work if I were to implement it? Let's compare a normal function call to one using FFI:



fn add(a: i32, b: i32) -> i32 {

&#x20;   a + b

}



fn main() {

&#x20;   let x = add(5, 3);

}



When the compiler runs the function call, it already knows;



Where the function is



Its' paremeter types



Its' return type



That the calling convention is according to Rusts' ABI (Application Binary Interface)

Another new word. Basically the ABI handles how compiled code talks to other compiled code in memory. Not so basically, this means

that the ABI of different languages do the following things differently:



How function paremeters are passed



How return values are delivered



How structs (objects, classes and stuff) are structured in memory



How function and variable names end up looking in binary (also called "name mangling")



Further calling conventions such as how the stack is cleaned and just how a language looks in memory



260629



Today I have started researching and writing a few lines in Rust, or more specifically FFI in Rust. I think that the lines of code I

wrote gave me a pretty good introduction to it all.



The first line in my code is something called an "attribute" and apparently it makes it so that the remainder of the script runs under 

some form of attribute.

The attribute I had set was a linking against windows library "advapi32", a DLL.



It's in this linked DLL that I'll access the functions that expose data gathered by ETW.



And how I run the functions isn't exactly what I'd guessed but I've seen something similar before. It's basically the same as a header

in C/C++, just that it's not in a header file.



How it works is that you write the function signatures for all of the functions you'll be using aka. you "declare" all of them

(function signature is the name, paremeters and return type) and during linking if the function symbol (just the name) between a

function signature you've written and a function that exists in a linked library matches, calling the one you've written

will call the actual complete one.



Think about it like this:



You declare the signature

int Add(int a, string b)



You compile so that LINKING occurs



The LINKED library has function

int Add(int a, int b)

{

result = a + b;



return result;

}

(notice how the actual logic is in here)



The LINKER notices that the symbols between your function signature and the linked function match (the symbol is Add)



Now, calling

int Add(int a, string b)

in your script will run

int Add(int a, int b)

{

result = a + b;



return result;

}

Even though the paremeters don't match, because only the function name aka. the symbol has to match



This will cause an error in the linked librarys' function



Update: Upon further looking into it, most languages have function symbols as both the name and paremeters in some format, but in C

the above explanation is accurate. And C is the only function type I've linked towards so far. This is not that important tbh.



Either way, at the point of writing this I have paused just before getting to the part of writing the actual functions and before

continuing coding I think that there is one more important aspect that I should write about and that aspect is that the linked

functions are in another language (or atleast using another ABI).



In my code, there is one line that accounts for this and that is:



unsafe extern "system"



This line is followed by two brackets in which all of the function declarations are to be written. What the line actually does is

to handle all of its' content according to another ABI, in this case that of the system.



The "unsafe" notation was automatically added upon writing the latter two words for me. Probably on behalf of the fact that the

linker might later return errors if I've mistyped any function declarations.



A breakthrough that I just recently got is that a function declaration is completely different from a function itself. One is no

more than a request and the other is actual logic, so when I declare a function for myself nothing is really created in memory for

a linker to find a match for. The only thing that actually is created is a request for what I am asking for.



In more concrete terms, extern "system" doesn't lay out incomplete functions in memory and then link them towards their full forms.

Rather, requests are sent in the language of "system" and that's it, the linker handles the rest. I just feel like declarations

could be less similar to complete functions because of how different they really are in what they do.



Also, because of Rusts' name mangling, its' way worse at being linked towards than something like C which has much simpler function

names in memory. Even though Rust can apparently unmangle using the attribute #\[no\_mangle]. Kinda goofy.



We're in notepad because VS Code won't stop crashing. I really hope it's not because of that non-working word count extension I

installed.



260701



Switched to markdown file.



Anyways, seems I left off trying to understand how linking works in Rust and how to set it up. Now it has come to writing the functions in my Rust code.



So for a function to be properly linked, the signature has to match the one in the library I'm linking against. And yes because it's

C it would also link if only the function symbol matched but then calling it later wouldn't work because of passing wrong argument

types.



So C has ints, floats, strings and all and Rust also has those and even though they aren't laid out the same in memory, they represent the same thing.

For example if you make a string "potato" in C it might look like this in memory (it doesn't, just an example):



1-P-O-T-

2-A-T-O-

3-------



and if you make it in Rust it might instead look like this:



1-XSLAE-

2-3-5-9-

3-1-----



But the point is that they're both in one way or another interpreted by the compiler as "potato".



So when you're linking in Rust against C, you make not the exact same parameters but the corresponding parameters, which is all that matters because

later, you'll be passing Rust floats into C floats and Rust strings into C strings etc.



And yeah the translation obviously happens thanks to the function declarations being inside of the "extern "system"" scope which enables linking in the

first place.



You know sometimes I find myself struggling to return to projects and I feel like a big reason for that is because I convince myself that I've forgotten

crucial parts since last time. Like I feel like an imposter every single time. And I feel like todays journaling kind of shows that I burn myself out on

proving that I'm still worthy of continuing the task. But I have to learn to trust my inner self, that progress will stay even if I sleep and take breaks.



So let's get these functions running.



Now, the work is progressing towards specific implementation and away from learning the fundamentals. Up until now the ways in that I could have found the

information that could teach me was quite widely available on both youtube and LLMs and all that but now it's getting more specific, this might allow me

to make real choices regarding how my code should look, rather than just finding out what to write and then learning about it. Hope I'm making sense.





