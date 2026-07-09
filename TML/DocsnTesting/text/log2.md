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

return a + b;

}

fn main() {

let x = add(5, 3);

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
names in memory. Even though Rust can apparently unmangle using the attribute #[no_mangle]. Kinda goofy.

We're in notepad because VS Code won't stop crashing. I really hope it's not because of that non-working word count extension I
installed.

260701

Switched to markdown file.

Anyways, seems I left off trying to understand how linking works in Rust and how to set it up. Now it has come to writing the functions
in my Rust code.

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

And yeah the translation obviously happens thanks to the function declarations being inside of the "extern "system"" scope which enables linking in 
the first place.

You know sometimes I find myself struggling to return to projects and I feel like a big reason for that is because I convince myself that I've 
forgotten crucial parts since last time. Like I feel like an imposter every single time. And I feel like todays journaling kind of shows that I burn
myself out on proving that I'm still worthy of continuing the task. But I have to learn to trust my inner self, that progress will stay even if I 
sleep and take breaks.

260703

Fixed the weird formatting after committing (?)

260704

Two words that I sort of thought were the same thing: references and pointers. But thats's not the case.

A pointer is what I thought they both were in some form, a number which in some way gives away the adress of the object it's refering to,

A reference is more like making another variable for a variable but under a different alias, like say you have very significant variable "x", you 
might then make multiple different references to that variable named "y", "z" and "a" each with their own use but modifying the same variable 
underneath.

Actually think about it this way, Terry is a very special guy with multiple occupations and hobbies:

So we got
struct Individual{}
and
let Terry = Individual{};

But we also got
let CircusTerry = &Terry
and
let SurferTerry = &Terry
and
let LifterTerry = &mut Terry
Any version of Terry is Terry himself
My analogy would be a lot cleaner if Rust allowed for more than one &mut at a time (mutable reference)

And a pointer to Terry would just be like 952 031 ChadHouse lane and in code it'd look like this:
let TerrysAdress: *const Individual = &Terry

So even pointers use the & reference syntax. References are actually much more common in Rust than raw pointers because the memory safety of pointers
 isn't automatically validated by the compiler while that of the references is.

Also
let TerrysAdress: *mut Individual = &mut Terry
Doesn't make it so that the pointer can be changed to a different type let alone anything at all, it only means that you can mutate the pointed to 
variable. Just confusing to me because "mut" has to be in two different places, but a way to think about it is that "*mut Individual" expects a '
mutable reference of its' type.

And all of this fuzz about a reference being a pointer but with "extra flavor" I think is just thinking too much about C pointers. A reference can be
formed using a pointer to refer to the original variable but it can be referred to in other ways as well.

-----

One more thing that I found interesting because it might present a problem later on was memory allocation before calling these windows functions.

I think everything that's worth going into is that:

let mut buffer = vec![0u8; 4096];
Looks real intimidating but simply allocates 4096 bytes of memory on the stack

buffer.as_mut_ptr()
Then gives the adress to that stack

So like if something says "Hey I need 4096 bytes to lay all this info out" you'd allocate it first and then pass the pointer. Same as how you'd free
 up space and then say where that space is so that things can go in there.

The data is allocated in a memory region called the "heap" which is basically one of the two most important memory regions to know about, the other
being the stack. I basically know how the stack works but I don't wanna explain it, the heap however is for more permanent memory which remains 
relevant for more than just one function call.

Another way to allocate memory on the heap is by boxing it.

Boxing a value is just allocating a single value in memory and then having a pointer to it, or a "smart" pointer because it cand do things like 
modify the allocated value without dereferencing manually. In code it looks like this:

let x = Box::new(5);

Here 5 is being boxed and the box is x.

-----

Knowing all of this I should be decently prepared to write the FFI ETW functions, right?
...
Okay so it's not really as obvious as I thought what I am looking to utilize of ETW right now. After thinking over it I think my approach right now
should be configuring recieving notifications from ETW. The reason why I'm not approaching starting a ETW session immediately is because it's another
big part of the project which I haven't prepared for yet, making it start as early as possible and all. For now I might start an ETW session
normally just to debug but that's
it.

So the question for now is "How do I get data from an ETW session subscribed to WindowsKernelProcessProvider and interpret it in my code?" This
 question sorta falls back unto what I've been studying.

260705

So ETW seems to have two distinct roles, and those are;

1. Producer / Session management
Where I create a session and subscribe it to the events that I want data on

2. Consumer
Where I connect to an existing session and recieve the data before "decrypting" and interpreting it

And the part that I'm gonna do first is part no. 2

Okay so getting into it, a few things are coming to my attention, such as:

The functions that I am writing consist of many structs which themselves consist of structs which also consists of structs etc. so I'll need to 
define all of them in a recursive order, this I'll do in a separate script called "FFI_Structs.rs".

Another script that I'll make is one that I'll call "FFI_Types.rs" and in there I'll write something called type aliases. It's quite simple and quite
useful but I've never seen or heard of it before. How it works is that it allows you to assign existing variable types new aliases, in other words you can say
that DWORD is just another way of saying u32. What makes this super useful is the fact that I can now just write the exact Windows Function arguments
instead of looking fot their Rust equivalents.

And a third script I will make is called "FFI_Constants" which contains just what it sounds like it does, constants which will be used in the
functions in different ways. The reason for constants needing to partake in the functions seemed unclear to me at first but it seems that they are used as
something called "flags"; Think about the arguments sent to a function as a car rolling into a workshop, the flag in this case can be thought of as a literal flag
attached to the car which tells the mechanic what to do with it.

Apparently Rust by convention follows "snake_naming". I'll sorta follow it in my file names.

Okay I've written the structs (or rather copied them from ChatGPT but still. I get what they do). How it should work now isn't exactly as I thought.
I won't be passing these as arguments to functions, rather;

I will use the structs to accesss certain data from a pointer that I'll get from Windows. To explain through an analogy:

unsafe extern "system" fn callback(
    record: *mut EVENT_RECORD,
)

Is the function that is run to get me a pointer to a place in memory generated by ETW, or rather a "buffer" where data is updated every so-often

A mutable pointer to a struct of type EVENT_RECORD

It is a mutable pointer because the buffer location might change (I think) and yeah it's a struct type that I've already declared. A struct consisting of many other
structs.

So because I have a pointer to a struct and I've laid out in my code how that struct looks, I can now access any data within it. Simple.

For the above function I just made another script and I admit it's hard to keep track of everything right now. The new script is currently called
"ffi_ETWConsumer" ("currently" because I'd see why I'd want to update that naming convention in the future).

The purpose of this new script is to do basically what I thought the "foreignFunctionInterface" script would be used for earlier but the project structure is looking
a little different than I predicted. It's kind of easy to guess what "ETW-Consumer" will do at this point. But to be clear it is explicitly to GET the information
that ETW provides, in supporting use of most of the other scripts.

Two completely unrelated things. First, I think a lot of the foramtting issues that seem to just appear every now and then is just because of me zooming in and out and the
text getting weird because of that. Second I have configured hackatime now so my time logging is from now on not in vain. I think just typing is justified as working on my
project... they don't really like fraud so I hope I'm okay. I will try to think about being tracked as little as possible so that it doesn't stress me out. Actually, they
seem to be pretty lighthearted about what "real coding" is. Cursor movements, switching between files and obviovusly typing all count as "heartbeats" of the project
moving forward.

Now to get the event-record pointer.

So what I said earlier about the "ETW-Comsumer" script and how it does what I thought the ffi_functions script would do. To expand upon it further, I earlier thought that
ffi_functions would serve the explicit role of gathering and interpreting data from ETW and that all of the other scripts would be in support of this task. But that is not
the case. Rather, ffi_functions only serves to directly communicate with windows, so I was right about it containing functions linked for windows but it does not handle
the actual interpreting part. That is up to the "ETW-Consumer instead. That is how the layout in my head looks so far.

So NOW onto writing the function within ffi_functions to get the event-record pointer.

Actually, to rearrange the mental picture again; ffi_functions explicitly sets up the connection with ETW and the ETW-Consumer now renamed to ffi_EventDecrypter then
repeatedly reads that data. And also I've been thinking of the script called "foreignFunctionInterface" as one called "ffi_functions" this entire time. But NOW I am back on
track!

So NOW onto THAT thing I've confidently explained twice before.

Another clarification, but now regarding the said "function that gets the event-record pointer". It's not the case that I get the pointer but uhh..
Yeah I'll take it tomorrow.

260706

I think taking what I was struggling with yesterday from a perspective of looking at the code instead of understanding the abstract idea will help me understand it better.

So I DO get a pointer to a place in memory containing the ETW information, I get that pointer by running this linked method (which currently exists within ffi_functions,
as per its' supposed use):

pub fn OpenTraceW(
    logfile: *mut EVENT_TRACE_LOGFILEW,
) -> TRACEHANDLE;

Tracehandle is just a u64, however EVENT_TRACE_LOGFILEW is a whole new very large struct which in itself contains at least two more recursive structs. A recursive struct
is a word I've come up with to explain a struct inside of aother struct which itself may consist of further "recursive structs".

Yeah so I ask ChatGPT to declare the new struct and its' structs in Rust, a fairly large waste of time otherawys in my opinion.

Actually, no. It seems that I have pushed ChatGPT to its' brink, it literally told me "i don't want to do that" because that would mean translating dozens of recursive
Windows structs from C to Rust. So I can't manually write all of Windows functions, this means that I'll just have to go with a less barebones alternative. Luckily, many
altarnatives exist and are often much more commonly chosen over what I was aiming to do.

So you know headers? That only exist in C/C++ to my knowledge and are just what I am missing out on right now, well they do exist in C for the functions I want to link
against, and even better there's a tool that lets me translate headers into other languages. It is called "bindgen".

There's one more even less barebones, more meaty alternative. It's also the alternative that presented itself to me first and which I proudly steered away from, it's the
"crate" alternative. It's also the alternative which in costrast to my earlier belief happens to be the most reliable. This is because of instead of me translating the
headers from my current Windows version into a script full of FFI functions, I immediately install a "crate" which is an actively updaetd and managed translation of the
raw Windows SDK. I think I will be using this alternative actually, as I said earlier it's pretty much a waste of time after I've understood the fundementals.

So here I will finally use the Cargo.toml file, not to actively refer to in order to run functions but just to install the proper crate from the web. That's how it works
apparently.

Straight to adding the crate, I asked ChatGPT to lend me hints on how to do it instead of me just copying everything straight up, just for me to get an understanding of
how Cargo.toml works.

So I know that it's basically a web available package and I also know that it's "modular", which means that I don't actually install everything "Windows" but instead I
just ask for the thigs that I need.

My guess is that there's some sort of official online Rust package manager where I'm given some sort of signature to put into my Cargo.toml file and then I add an
additional signature to determine what parts of the package that I get. How exactly the package is accessed I'm not too sure, it'd be weird if internet access was needed
every single time but it'd also be weird if Cargo.toml was needed only for the initial download.

Asking ChatGPT about this line of thinking, my suspicions are largely confirmed. The official site is called crates.io and I write in the Cargo.toml file what parts of
the crate to actually include.

Actually the site docs.rs also exists, here the original crates aren't actually kept but they are explained and commented on. And I also found it easier to find the
crate that I was looking for here.

Holy sh*t there's alot of windows stuff.. No way all of this is necessary.

Also the looking between packages inside of the site was easier but otherways the explanation and everything else inside of Crates.io is way more comprehensive, at
least for this crate as it seems that the "comment section" part of it at docs.rs was unavailable.

Alright I found the initial signature for the crate but I am struggling to find the one for ETW.

With a little help from ChatGPT I found the contents of the crate that I've been looking for. Not that it means much but on the code page in crates.io it's in the
directory "src/Windows/Win32/System/Diagnostics/Etw/mod.rs" and the mod.rs is apparently just all of the contents of ETW as ETW here is just a folder as everything before
it. This is very exciting but I still don't know what "sub-signatures" to put into Cargo.toml, because I know that it's not supposed to consist of slashes.

After clicking through into the same directory but on docs.rs I was met with "windows::Win32::System::Diagnostics" which is the same as before but with double colons and
without "Etw" on the end. So I'll paste it into Cargo.toml:D (with Etw too of course)

So apparently how crates in general work is that you run a command once in the terminal to install the needed package in use of Cargo.toml, instead of needing internet
everytime. Makes sense. And how this crate works is that not just the functions is included but everything that I've basically already written, which is okay. I haven't
really done any time consuming setup things so far.

So included in the crate is:

Type-aliases

Functions

Structs

Constants

Enums(?)
(From what I understand an Enum is basically a variable with a few different states that it can take on and those states have to be accurate in order for me to intergrate
them)

And that is it I think, I already have a decent understanding of most of these so I don't see the problem of taking the (clearly recommended) shortcut.

Now onto the command to run in the terminal, looking for it online had me worried for a moment because they were talking about "Cargo new" as if I was supposed to know
what it was but all it does is create the folder structure that I've already created so we good. Anyways the command for getting the crate is "Cargo build" and powershell
is giving me an error for the "Cargo" command not being found which is understandable because the only Rust-related thing I've installed is the extension for VSCode.

Looked it up and configured rustup on my device. Seems right I think. Now Cargo runs but I'm getting error "string values must be quoted, expected literal string". Fixed,
had to put "windows::Win32::System::Diagnostics" inside quotation marks. Now getting error "this virtual manifest specifies a `dependencies` section, which is not allowed".
Asked ChatGPT and apparently there are "two different kinds of Cargo.toml" and of course I am using the wrong one, I don't think that makes any sense but ok. 

So because I don't have any experience using Rust I just ChatGPT'd the errors. I was given this:

[package]
name = "TML"
version = "0.1.0"
edition = "2024"

[dependencies]
windows = { version = "0.62", features = ["Win32_System_Diagnostics_Etw"] }

Which does three things. First it replaces [dependencies.windows] with [dependencies] which definitely makes more sense to me since it's supposed to be a category inside of
Cargo.toml and not a part of the crate. Second it changed the version ">=0.59, <=0.62" as recommended by Crates.io to just "0,62" (This could probably be a case for errors in             \\//
the future so I guess I'll mark this place in the log somehow). Third it added the package part which specifies the custom name of my project, the custom version of my project             \\  
dependency-wise and also the version of Rust that it uses in one way or another, that way or another being that you can only run main otherways so pretty important actually.              //\\

And yayyy Cargo build worked!

260707

So as I wrote in my previous commit, now all of my scattered Rust files actually count as a Rust project with the addition of the built Cargo.toml file as well as the newly
added dependencies.

So now that as I previously mentioned, all of the types, functions, structs etc. are in place, it's time to get around to utilizing them. I also believe that I in some point in
the past specified what this step would be about, that being that I'd first get the pointer to the ETW buffer and that I'd do that inside of the "ffi_functions" script which was
supposedly reserved for linked Windows functions, and that I'd then continue to further decrypt the data in that buffer in the "ffi_EventDecrypter" script (this scripts name has
changed many times so maybe it's hard to me writing about it earlier in the log).

Actually, looking back at this explanation it didn't really make a lot of sense to organise the "course of action" across scripts as I should actually only declare in those
scripts what I'll refer to through main. And obviously right now I also just refer to everything from main, so it's less about "do this here and that there" and more about just
preparing the proper tools in different scripts, or at least it was about that before when I had to like link and declare everything concrete manually but now that came in the
crate instead, I guess my job becomes more about like configuring the later use of the complete setup which might allow me to explore a few different alternatives.

Holy yap sesh

-----

Although before continuing further I'll check whether the crate I added is working. I do this by simply referring to the crate in main and compiling my project which I do by
running Cargo run in the driectory of my Cargo.toml file and src folder. I was met with the following error:

"
   Compiling proc-macro2 v1.0.106
   Compiling quote v1.0.46                                                                                                                                                                                                                  
error: linker `link.exe` not found                                                                                                                                                                                                          
  |
  = note: program not found

note: the msvc targets depend on the msvc linker but `link.exe` was not found

note: please ensure that Visual Studio 2017 or later, or Build Tools for Visual Studio were installed with the Visual C++ option                                                                                                            

note: VS Code is a different product, and is not sufficient

error: could not compile `quote` (build script) due to 1 previous error                                                                                                                                                                     
warning: build failed, waiting for other jobs to finish...
error: could not compile `proc-macro2` (build script) due to 1 previous error                                                                                                                                                               
PS C:\Users\nora.svahn\Desktop\TML_repo\TML\DocsnTesting\code> 
"

So this error message mostly consists of lines "note:", "error:" and "warning:" besides the weird stuff happening at the start and the project directory at the end. I don't think
it's worth delving into every error so I'll just look for the quickest solution.

260708

Next day and I found the solution. It was that I had to have had the vsc c++ package thing on visual studio installed so I got that done.

Just one more error came up after that and that was because my main.rs script didn't have a main() function.

Now it compiled though.

To speak of all the dependency files that Cargo.toml adds to my project when "cargo run" or just "cargo build" is run, I am apparently supposed to put it in .gitignore because each
time that my program will be run on a new device all of those files should be installed again.

That makes it make more sense to me.

Before I was confused about why you wouldn't just install the dependencies once and be done with it instead of configuring it through some special file but now it makes more sense.

It's basically a note which contains a more abstract version of what is needed for a program to run, so that; instead of every possible solution being installed initially, just the
single one thats necessary can be installed.

Also including .vscode in my gitingore

And of course running git rm -r --cached on both of them

the only generated thing that should stay in the repo that comes to mind right now is Cargo.lock, its' role appears to be to ensure that everyone gets the same version of dependency

What's next again?

I guess getting into the logic

All of the scripts that I've made could definitely still be put to use, even though I installed a crate for all of the dependencies, or some of them at least. I'll probably end up
reorganizing a lot on the near future, like around decrypting the pointers' data and all

Using scripts other from the main script is done by typing "mod [scriptname];" at the top same time as it's in the same directory. This isn't necessary if it's a crate added with
Cargo. Something that's true for both cargo crates and same directory scripts though is that you can typ "use [script/crate]::[further-specification]::[function];" at the top of the
script to not have to write "[script/crate]::[further-specification]::" every time you use [function].

Removed every script except for main.rs

Now to get that pointer to the etw data, get the necessary info out of it and then eventually decrypt it into something that can be used to make ui

260709

Something that I have not accounted for in the past is that the "linker" has to be installed and "linking" is not a default part of compilation. Like how I have to install some kind
of microsoft linker in order to run the etw functions.

Now to maybe finally get to what's 7 lines further up

It seems like the etw data is delivered inside of an "EVENT_RECORD" struct, and something that I seem to have understood correctly is the fact that I am given a pointer to that
struct which afterwards enables met to decipher it.

Now how I get the pointer is that I write a function myself that can be called whatever but with an "*EVENT_HEADER" argument, and then I pass the adress of that function to windows
which allows the system to call it.

How I pass the pointer of the function to windows is that I