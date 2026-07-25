<260627

So I accidentally forgot to save the log that existed before I started writing today but it was mostly about me finding out about the
different alternatives that existed for gathering process data and then narrowing it down to ETW instead of like WMI or
CreateToolhelp32Snapshot because WMI unnecessarily "wrapped up" and the latter is unoptimal for continously tracking processes. ETW
on the other hand is basically exactly what I was looking for, a low-level system-intergrated API which runs continously and returns
lots of rich data.

And I chose Rust for the language because I wanted to (even though I've never used it before)

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

Now how I get the pointer is that I write a function myself that can be called whatever but with an "*EVENT_RECORD" argument, and then I pass the adress of that function to windows
which allows the system to call it.

Actually a very good way to think about interacting with windows is that you basically make a large struct with a bunch of information which you then send into the system so that it
basically works as a construction manual for how windows should use your code in its' own workflow.

And one of the pieces of information in this large struct is this function you make where Windows funnels the necessary information

The alternative to the this model would be just making a function with as many arguments as the struct has variables

And in this model in my case, that "construction manual" is:

"
let mut logfile = EVENT_TRACE_LOGFILEW {
    LoggerName: default(),
    LogFileName: default(),
    CurrentTime: 0,
    BuffersRead: 0,
    LogFileMode: EVENT_TRACE_REAL_TIME_MODE,
    LoggerThreadId: 0,
    LogFileHeader: std::ptr::null_mut(),
    BufferCallback: None,
    BufferSize: 0,
    Filled: 0,
    EventsLost: 0,
    EventCallback: None,
    IsKernelTrace: 0,
    Context: std::ptr::null_mut(),
};
"

And in this large, theorejhtical manual with a bunch of different checkboxes and lines to be filled before being passed to the system, my function goes into the section
EventRecordCallback, thus:

logfile.EventRecordCallback = Some(callback);

And something very convenient about filling out this manual is that you are able to only fill in the fields that you care about, using a feature called "the default trait" where
every variable of a struct has a "default" value if the trait has been attributed to the struct.

So as it seems right now, I shouldn't have to account for dozens of struct variablesd just to be able to get the etw data

So instead of writing what I did earlier, I can just write:

"
let mut logfile = EVENT_TRACE_LOGFILEW::default();
"

Here I don't have to declare the type of logfile before the "=" mark thanks to "type interference". Because it can only possibly be one type to store the type on the right of the
mark

I assign the variables that matter afterwards:

"
logfile.EventRecordCallback = Some(callback);

logfile.lalala = 5;
"

260710

It seems that there are only three variables that i have to alter in the defaulted struct in order for the etw interaction to work at an absolute minimum. These are:

LoggerName - The name of the etw session I am connecting to, according to windows syntax so it's different from a regular Rust string. Elaborated on later

EventRecordCallback - The pointer to my EVENT_RECORD callback function. Elaborated on later too

ProcessTraceMode - This variable determines some of how the interaction between my code and etw will work, it's a flag variable, two of which I have to alter.
I never really got around to understanding how flags worked other than that they basically served as signatures for determining how a process would run, but they're very simple;
Flags are usually stored inside of a uint and each flag takes up only one bit, meaning that if you're using a u16 you can basically store 16 bools inside of it. So flags are like
bools but much more versatile, or at least more memory-friendly.

The two flags taht I have to change are:

REAL_TIME - Says that I am consuming a real-time session rather than reading en ".etl" file (Which I guess is like a stationary file containing similar data to a live etw session)

EVENT_RECORD - Says that my function is using the more modern "*EVENT_RECORD" argument rather than the older ""EVENT_TRACE"

And because I already have the flag constants declared in the crate, I already know what bits that the each respective flag represents so I can just run the following:

"
logfile.ProcessTraceMode =
    PROCESS_TRACE_MODE_REAL_TIME |
    PROCESS_TRACE_MODE_EVENT_RECORD;
"

Making the two bits be modified together:

"
0001
+
0010
|
V
0011
"

Assigning my functions' pointer to EventRecordCallback is written as:

"
logfile.EventRecordCallback = Some(callback);
"

The "Some(...)" is explained a few lines down

Right now I have a pretty clear understanding of why the whole thing works, like the struct "EVENT_TRACE_LOGFILEW" is written according to the C syntax in the crate and I assign
the name of the etw session in the correct syntax as well as the flags in the universal flag syntax. I don't understand however either what type of variable that EventRecordCallback
is and why I assign it as stated above.

Apparently it's declared as follows in the crate:

"
pub EventRecordCallback: Option<
    unsafe extern "system" fn(*mut EVENT_RECORD)
>,
"

Option< ... > - Means that whatever is declared inside of its' scope is optional and it can also just be none, assigning a variable to an "Option<...>" must be done with "Some(...)"
as shown on line 648. "Some(...)" isn't used to declare the two other obligatory variables because the flag is always an integer, even if all of it's flags are off it's an integer
showing 0 and there has to be an etw session name so there is no option for it to be null either.

unsafe extern "system" fn(*mut EVENT_RECORD) - Means that the option other than nothing is a function following the system syntax and with the argument of a pointer to struct
EVENT_RECORD which.. is a struct that follows the system syntax as well but not because it has to be to communicate with the system but because it comes from the system

So onto making the string that is to be the name of the etw session that I'll be consuming. So apparetnly the weirdly placed W at the end of for instance EVENT_TRACE_LOGFILE happens
to explain what type of string the function or struct uses. In this case, the W stands for wchar_t which is a type that takes up 16 bits of space in C and is short for "wide
character type" and takes up twice as many bits as a normal char. The entire string isn't stored in the wchar_t however, only the first letter, therefore you pass a pointer to the
first character of the string in memory into LoggerName and then etw will keep reading those wide characters until it reaches a null terminator, aka. an empty memory to mark the end
of the string. To make said string in C wchar_t syntax in Rust I run:

"
let session_name: Vec<u16> = "MyEtwSession"
    .encode_utf16()
    .chain(std::iter::once(0))
    .collect();
"

Which is pretty interesting. The simple part is defing a vector of type u16, and each wchar_t in C works like a u16 in Rust. The following things are basically method calls on
that string before it is stored inside of the Vec<16>.

But a string obviously can't be directly stored inside of a Vec<16>, so I call another method on the string to modify it so that it it is converted into the utf16 format (UniCode
Transformatoin Format).

The only thing missing for C to be able to read it is the null terminator at the end which is fixed by the ".chain(std::iter::once(0))" which chains one extra 0 at the end.

And then to finalize the variable, you run ".collect();" to store the iterated value as the type which was initially declared, in this case a Vec<u16>.

This is interesting as it exposes low-level memory-representation for something as simple as declaring a string, fundemental because of interacting with other languages in FFI.

And to finally assign this beautiful string to the struct:

"
my_logfile.LoggerName = etw_session_name.as_ptr();
"

Simply meaning the address of the string in memory

260712

2 hours later, been coding for a little while without logging.

I'm done configuring the logfile and I have sent it in, also created a new script called misc.rs and made a function which converts strings from the Rust format into the C format.

Oh yeah I also had to include another "module" in the crate I guess you call it, to get the EVENT_TRACE_LOGFILEW struct. Took a while to solve, but now I know the error for it

Besides that I've also learned about "unions" in C, which are basically variables which could be different types and you can access them like you would any other variable of a
struct in C but in Rust you have to explicitly reference the union as well

The next step is kinda foggy right now but I guess it would be starting an etw session and trying to connect my code to it, maybe even configuring the a scheduled session at boot
time and having it write into an .etl file which I'll then configure reading for myself. I think working with the data is going to come quite late and it's at that point that I'll
know what more to do besides the minimal setup I have going at the moment.

-----

Having configured the minimum etw consumer, it's time to figure out the etw session

For now I'll just be trying to start one normally but later I'll try to schedule one for the following boot.

So a session is simply "a kernel object handled by windows". It has a name, a handle, a few settings and zero or more enabled providers (the "notifications" that etw recieves)

Okay so it seems that practically the most important thing for me to cover right now is the struct EVENT_TRACE_PROPERTIES and how to set it up. I'm glad I did the consumer part
first because this one seems a lot more complicated at first glance at least.

260713

So as told previously, there are clearly similarities between EVENT_TRACE_PROPERTIES and EVENT_TRACE_LOGFILEW. The difference is that the former is about configuring the session
itself and the latter about configuring the connection between my code and the session. Another similarity is that I'll be calling a function in regard to that created struct
and for the logfile that was OpenTraceW and for the properties it's going to be StartTraceW that I'll be using to "register" the struct to windows.

Actually there seems to be another important struct when it comes to starting sessions, it is called "ENABLE_TRACE_PARAMETERS" and it takes on a very prolific role. It also
mechanically has a function for passing it that is called EnableTraceEx2()

If EVENT_TRACE_PROPERTIES is a (pretty advanced) radio which can access any frequency at once and passing it with StartTraceW is turning it on, EnableTraceEx2() is telling
radio stations to broadcast to this radio and ENABLE_TRACE_PARAMETERS contains further, more advanced instructions about that particular broadcast.

And the workflow of creating a session is going to be calling StartTraceW() with the trace-properties struct (EVENT_TRACE_PROPERTIES) and the configuring the events it is going to
be tracing by separate EnableTraceEx2() calls with the trace-parameters struct (EVENT_TRACE_PARAMETERS).

So lets' start with the trace-properties struct.

Now the weird thing is that I'm not only passing the struct but a whole memory region which contains the struct among other things, the first thing I do is get the size of the
struct in memory by using std::mem::size_of which gives the size of the member in bytes and then I make a buffer which is the size of it and then some

"then some" being exactly the length of the name of the session that I am creating times two, don't ask me why yet

And then what you do is the following line:

"
let properties = buffer.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES;

"

Which passes a pointer to the struct into the start of the buffer and because it's a fat pointer it also contains information about how to handle what it points at, if it just
would've been left as "buffer.as_mut_ptr()" then interacting with the pointed at value would be like interacting with an individual byte, because the buffer is essentially a vector
of empty bites as per;

"
let buffer = vec![0u8,buffer_size];
"

where "0u8" stands for "00000000" aka 8 bits aka one byte

by then writing "as *mut EVENT_TRACE_PROPERTIES" you say to treat what the pointer is pointing at not as a byte but as a you guessed it trace-properties struct, and it contains a
second "mutable" because a mutable pointer is what the earlier ".as_mut_ptr()" expects to stay as.

But the allocated memory region that is being pointed to is still empty. Luckily, the pointer to it which I just explained is mutable which means that I can edit the memory through
it. Remember how I said that the size of the buffer in memory is the size of the trace-properties struct plus the length of the string that holds the name of the etw session I am
creating times two? Well apparently it's because the buffer is supposed to be laid out the following way (creds to the big GPT):

"
+----------------------------------+
| EVENT_TRACE_PROPERTIES           |
+----------------------------------+
| "MySession\0" (UTF-16)           |
+----------------------------------+
| (optional log file name)         |
+----------------------------------+
"

And the pointer I've made makes it so that interacting with the first part of the buffer which takes on the length of EVENT_TRACE_PROPERTIES works the same as interacting directly
with EVENT_TRACE_PROPERTIES which is hella convenient. Or not exactly the same you need to include lil brackets like this:

"
(*props).Wnode.BufferSize = buffer_size;
"

Because it's not the pointer itself that has the struct variables but the struct that it is referencing

"
*props.Wnode.BufferSize = buffer_size;
"

Would be an error because it'd be like modifying the pointer itself because it only contains an address and nothing else, contrary to my earlier belief that it also contained data
on how to modify the referenced type though I now understand that that data comes from the struct itself. Further evaluating this idea, there aren't as many cases as I thought where
fat pointers are actually needed, they ARE only necessary when the pointed to type doesn't contain enough information about size or usage or something like that.

Anyways now that I've finished assinging the variables to the imaginary struct, it's time to assign the one other necessary thing at the moment to this memory region, the session
name. This is done by first getting the pointer to the location that it is supposed to be in and then running the function:

"
std::ptr::copy_nonoverlapping(
    session_name.as_ptr(),
    string_ptr,
    session_name.len(),
);
"

Which is basically the same as this (in C# because idk for loops in Rust):

"
for (int i = 0; i < sessionName.Length; i++)
{
    stringPtr[i] = sessionName[i];
}
"

where the indexes' of stringPtr refer to the amount of u16 from the start of it and the indexes in sessionName are the character in that space of the word.

260714

So coding-time-wise I am pretty much at the ten hour mark right now since I started using hackatime at least and only up to ten hours count towards the actual "ship payout" per devlog
so I'm not sure whether I should keep coding and logging until I reach a coherent "checkpoint" or if I should write the devlog about where I am right now.

I think the best option is to make it so that my code does what it's supposed to right now which is simply to

260715

Aaaannd I fell asleep, anyways two big milestones today:

I posted my first devlog on stardance !! Even though I had to post like 5 times because my initial devlog was too long and then I had to mess with the formatting and aahrarhab

But it's okay. Because my code successfully created an ETW session !!! After messing around a little bit with the StartTraceW() script it's actually running :D
What I remember from the "messing around" is understanding the difference between pointers * and references & further, for example this makes sense:

"
*mut CONTROLTRACE_HANDLE
"

but not this:

"
*mut handle
"

Because a mutable pointer to a place in memory is supposed to provide information on how to interact with that space, not on what variable is at it. Explaining why it's a "raw" pointer.
References on the other hand work the other way around, they "point to" explicit variables. And if a function expects a pointer as an argument, passing a reference is fine as well as
Rust is able to convert it because the variable that a reference is referencing also reveals the type that it is, thus what type of pointer that the reference should become.                       REFNPOI

Next up, StartTraceEx2();
What the function is supposed to do is enable an event provider for me etw session, the function call looks something like this:

"
EnableTraceEx2(
    handle,sdsds
    &SystemTraceControlGuid,
    EVENT_CONTROL_CODE_ENABLE_PROVIDER,
    TRACE_LEVEL_INFORMATION,
    EVENT_TRACE_FLAG_PROCESS,
    0,
    0,
    null(),
);
"

Which contains quite a bit more arguments than the functions I've written before, here's a summary of them:

handle - the handle that I get from StartTraceW() presumably.. Yup I was right, so it's the handle that I get to the session that I crate, makes sense

&SystemTraceControlGuid - A little whacky but this is a reference to a GUID which explains the category of events that I want my session to recieve, so because I want to recieve events
from the windows-kernel-process-provider I pass the GUID of basically "kernel providers"

EVENT_CONTROL_CODE_ENABLE_PROVIDER - I doubt there's a bigger fish, basically explains what I want that provider to have to do with my session, in this case it's to enable it. And yeah
it's not a type but a constant, a flag. Same as how every other variable in here isn't actually a variable but the rough value that I want to assign to the parameter in my case

TRACE_LEVEL_INFORMATION - The "verbosity" of the info I want to recieve, like how detailed I want it to be

EVENT_TRACE_FLAG_PROCESS - Only at this stage is it specified that I want to recieve events from solely windows-kernel-process-provider

The 0, 0 null() at the end are just additional settings that I don't need to mess with right now, it's a little reassuring though since it tells me that I am actually making something
grounded which I can cultivate understandingly later on.

Now even though I've said that the above function is made to enanble the process tracer provider, I think a better approach would be to enable the file tracing provider at first, as I
think that it would allow for easier debugging because there's presumably less files than processes being created and destroyed at any given moment. So I can just start my trace and then
create a text file or something and see if my callback function is called to make sure everything works as expected.

Don't think I've ever seen this many red lines one after another, well all of the constans autocorrected so those exist, hovering over the function it seems to mostly point out incorrect
types. It's a little confusing though since the constants have types that also sound like constants, like TRACE_LEVEL_INFORMATION is a bad example because it's just u32 but 
EVENT_CONTROL_CODE_ENABLE_PROVIDER has type ENABLECALLBACK_ENABLED_STATE, SystemTraceControlGuid has windows_core::GUID or more specifically
windows_core::GUID::from_u128(0x9e814aad_3204_11d2_9a82_006008a86939); and so on. Real messy.

Got rid of the errors by learning a new way to convert types. Basically if something is assigned as a struct in the windows crate but really just contains a single int which is what the
function that I am calling expects I can run SOME_WINDOWS_STRUCT.0 <- to get that int and then I can cast it to u8, u32, u64 as usual.

The reason that there are structs which just contain a single variable is not because of some mistake that the crate made translating the windows syntax, it's for the sake of the structs
working as something called "wrappers" or something like that and they exist so that you're not handling a bunch of different integers which you have to keep track of. Like imagine if
you had:

"
let important_data_1: u32
let important_data_2: u32
let important_data_3: u32
...
"

you'll probably mix them up, but if each type of important data has a distinct struct type like:

"                                                                                                                                                                                   WRAPPERSSS
let important_data_1: structx
let important_data_2: structy
let important_data_3: structz
...
"

where every struct is just

"
structx{
    int: u32
}
"

then they're way easier to keep track of since you know, you can look at both the variable and the type and know what it is

And something cool is that a u32 and the a struct that only contains a u32 often look the same in memory, so it's just the way that you're accessing them that is different. It's just
for Rust being so security-tight that you can't use structx in the place of a u32 and instead have to call structx.0 to access its' first field.

It seems that there has been a misconception on my end regarding how windows calls my function. It can't just call it directly because it's public and it has the pointer to it.

260716

Alot on unlogged work yesterday. Mostly because I'm struggling to understand how windows is accessing my function now that I've gotten to that point, as well as how it plays in with
everything else.

I think now that my code should be able to create a session, enable providers for it and listen to it that I should write down the mechanics for myself.        ETWEXP

1. So the first thing I do is just write the name of the session that I want to start

2. Then I give that name to my function that creates a session which it does by allocating a buffer and then filling it with the equivalent of a struct as well as the name of my session
and "optionally" the name of the file that it's supposed to write its' data into, and that "optionality" is configured by the contents of the struct. My function then passes that buffer
to windows.I pass a buffer and not a struct because it's more optional or whatever. Also, the function returns a handle to the session which will later be used to modify it.

3. I then also give the session name that creates a consumer for the created session, it's more simple, I just create a struct and pass it with a function instead of a buffer. I guess
that it's more simple because of the principle, it just listens to a configured session so it does not have to be as complicated. The only things that the struct contains are the name of
the session it's connecting to, a flag to configure what version of the session it is whether it's live or not and how modern it is and more importantly a pointer to one of my functions
that will be used later.

3 and 4 should probably switch places but whatever it works like this

4. I enable a provider for my session my passing my session handle into a function which only returns a message whether starting it was successful or not. No passing structs here but still
quite a lot of parameters on the function I'm using. The parameters pretty much consist of an unusally complex convention for getting the provider that I want as well as something that can
be optionally assigned in order to determine after how long the proivider is gonna stop sending notifications. So this basically modifies the etw process that I've already opened.

5. I guess this is the part that I am confused by. If I had to guess I would say that it's about configuring the existing listener session opened by opentracew or more like making it do
something. It works by passing the consumer handle into another function (as with step 2 together with step 4). The main thing that confounded me was how windows doesn't actually directly
call my function through its' pointer that I passed with opentracew, rather another function in my code that I am supposed to write called ProcessTrace() which is another linked function
USES the pointer that I passed beforehand in order to call my own function that receives the etw data, and something else perplexing about ProcessTrace() is that when I put its' decelaration
in my code it obiously links towards its' definition BUT because that definition consists of a while loop, ProcessTrace() now also works as a while loop in my code. And what that while
loop does is check the receiver of events created by OpenTraceW() every loop for new updates and when it does receive updates, it both sends a little status message on its' own and also
calls my function with the EVENT_DATA that it itself received.

A summary would be:

1. Write session name -- (Just declare a string)

2. Configure and create session once with session name - get session handle -- (Pass buffer containing EVENT_TRACE_PROPERTIES layout + offset of the string since start of buffer + optionally
file name string ++ an additional argument containing an additional reference to session name location, technically unnecessary but windows is old ++ an additional argument containing a
pre-declared empty handle StartTraceW() - get the now-defined session handle CONTROLTRACE_HANDLE back as well as a status message)

3. Configure and enable providers once for session using session handle -- (Pass arguments regarding which provider and its' verbosity as well as handle of the session its' supposed to provide
to through EnableTraceEx2() - receives status message)

4. Configure and create session-listener once for session with session name - get listener handle -- (Pass defaulted struct EVENT_TRACE_LOGFILEW with necessary values defined to OpenTraceW() -
get PROCESSTRACE_HANDLE consuimer-handle back and no status message since the handle is the return)

5. Continuously get information from session-listener using its' handle -- (Pass consumer-handle and optionally other arguments regarding the period it's supposed to send notifications through
ProcessTrace() which is a loop that call my function with the EVENT_DATA parameter fulfilled - ProcesTrace() also returns a status message each time it recieves an event)

(6.) Change or end the session afterwards using the session handle -- (By passing my session handle and what I want to change to ControlTraceW() - probably also returns an status message but
I haven't used it yet)

Additionally, more than one consumer can be active for a session at a time which wuld require stage 4 and 5 for each of them

-----

Now I've set debug prints across my code that work like this:

#1 containing starttrace status message and session handle

#2 containing enableprovider status message

#3 containing opentrace consumer handle

Then the processtrace status code along with hopefully a print from my callback function as well to confirm that its' been called

--

I'm getting status message 0 from the first thing I do which is creating the session, that error means that my code doesn't have permission to create a session which makes it weird how that
part worked on the school laptop but whatever

So how do I solve this? Just run cmd as admin.

-

Which I did and now everything is working except for that ProcessTrace isn't firing.

Could be because I enabled the wrong trace if even a valid trace at all, the Rust formatting on that part was really tricky so that'd be my guess. To be clear, the trace I wanted to enable
was the one that sends a notification each time a file is created or deleted and I did create and delete a file and got nothing from ProcessTrace().

I'm even sure that the above is the reason because enabletraceex2 happens to be able to still return status message 0 even if it enabled a non-existent provider I guess because it did so for
a session that does exist. Or I'm not sure if it can actually literally enable a provider that doesn't exist but the syntax for doing it is complicated to say the least and you don't get to
know much from what the function for it returns. I guess cause calling it is really open-ended

So imma try some different providers

-

Tried the .NET provider and IT WORKS !!! So the enabletrace call for the file kernel provider was probably incorrect, I also tried the kernel process provider and it didn't work either. From
what I understand, calling kernel-level providers requires more fields to be filled in the enabletrace functiont than calling "regular" providers like the .NET one, so that extra step is
probably related to why it's failing

Oh and also the println!() that I placed in the function containing processtrace never fires because it's AFTER processtrace which is a LOOP. Silly me.

So now to figure out how to enable provider that I want

--

GeePeeTee just dropped a massive bomb on me now that I can use the better model again. The issue isn't that I'm calling EnableTraceEx2 with the incorrect arguments, it's that;

I'm defining the buffer for StartTraceW() in the old school way, windows xp era type sh*t and back then ETW was only meant to dispatch events from a few different kernel providers which were
assigned INSIDE OF THE BUFFER in the EVENT_TRACE_PROPERTIES region. Because what need would there be to enable providers by GUID with a separate function if there are only under a dozen of them
when you could just make it so that they're enabled along with the session?

But then why did the .NET provider still work if I am using this old framework? Probably because EnableTraceEx2() was created to enable providers other than the main kernel processes.

But then why did geepiddydiddy give me a GUID for a kernel provider? Probably because EnableTraceEx2() was later expanded to cover kernel processes as well.

Checking and I was basically right

So should I go with the old school way of enabling the corresponding flags in the buffer for the kernel process provider or the new school way of using EnableTraceEx2()?

-

Even though it's more complicated, I think I'm gonna go with the second way as it obviously gives my project more potential, it revolving around ETW and all

To create a session that uses EnableTraceEx2 for kernel providers, I have to enable this flag to the pointer EVENT_TRACE_PROPERTIES in the buffer for StartTraceW:

"
(*props).LogFileMode = EVENT_TRACE_SYSTEM_LOGGER_MODE;
"

260717 and I ended up trying to understand the classic logger mode instead because the modern one didn't really work, below is the setup I've changed so far:

The following flags have been changed for etp in the buffer:

"
//Classic flags
    (*props).EnableFlags = EVENT_TRACE_FLAG_PROCESS;
    (*props).Wnode.Guid = SystemTraceControlGuid;
"

where .EnableFlags is said way of enabling kernel providers through the struct and Wnode.Guid is basically an identifies for what kind of session it is

and besides that you need a specific session name when using the classic setup, namely "NT Kernel Provider" 

At the moment I am receiving a bunch of notifications through my ProcessTrace and I don't know what they mean so I am pretty much fully using AI to understand them. I'm talking like

"
Opcode: 3, Version: 4, Task: 0, ID: 0 Opcode: 3, Version: 4, Task: 0, ID: 0 Opcode: 3, Version: 4, Task: 0, ID: 0
"

But it's actually according to the gameplan, understanding what these mean are for later. Right now all that I know is that I am in some way getting messages from the kernel process provider
but that are slightly incomplete, like they're only control events and nothing actually informative right now. And it's like that because even if the buffer i pass for session data is slightly
incorrect, it'll just run incorrectly rather than giving me an error which I guess is the consequence for working almost completely in unsafe fields.

Nothing has really changed in the OpenTraceW function or ProccesTrace though even when switching to the classic kernel provider method which is nice

-

Something that I can notice personally about the events that I am getting is that they appear in a burst of hundreds of notifications when I start the session and then like 5 happen over the
following 10 seconds or so and then it just goes quiet even when I am starting processes (which it is suppoed to track). What does this mean besides possibly what I stated above? Still don't
really know.

--

Okay I think ChatGPT is trolling but my code works now, after:

- Changing the provider from just the kernel process provider to both it and the provider for new threads by changes the flags in the buffer struct
Which I don't think did more than it sounds like

- Also printing "UserDataLength" which I haven't done before but it made gpt convinced that a line that I changed like half an hour ago was the thing that fixed my code while it was probably
just printing the thing that actually shows that my code works

So this is what I changed to in the said struct buffer:

"
    (*props).EnableFlags =     EVENT_TRACE_FLAG_PROCESS |
    EVENT_TRACE_FLAG_THREAD;
    (*props).Wnode.Guid = SystemTraceControlGuid;
"

instead of what's a little further up

---

And I didn't say so earlier but I do WANT to go with the modern version but atleast now I have something to fall back on incase it never starts working

260718

I've managed to get events coming using the modern session setup. The problem was likely that:

I had to enable one additional flag within props in order to receive kernel events or rather to be allowed to use the kernel event GUID, the flag was:

"
| EVENT_TRACE_SYSTEM_LOGGER_MODE;
"

Which makes sense because enableprocess in the beginning was only for events other than the ones that could be enabled using flags like in the classic version, so they likely added a flag to
props later that would allow enableproces to partake in kernel events as well. Basically marking the change from the classic session version to the modern one.

But the provider enabling is so weird, you can't just give one GUID but you have to give a GUID and then narrow the events that you'll receieve down by using keywords, that are written into
two different fields:

matchallkeyword
and
matchanykeyword

So I guess it's like if I'm in the roblox catalog and I want to search for a "blue glowing octopus":

So there are three separate "tags" for what I am searching for which is "blue", "glowing" and "octopus", if I were to search using matchanykeyword then I would get all items with either one of
the tags, so I would get everything from "blue jacket" to "glowing green eyes".

But if I'd search using matchallkeyword then I would get just items that have all of the specific tags even though they might have more than just those, so I'll only get "blue glowing flying
octopus" and "octopus on ship with glowing moon" which narrows it down a lot further as these would also have been included if I would've searched with matchanykeyword.

More specifically, because both of them are appplied together, the matchallkeyword is applied first and then additionally the matchanykeyword to expand afterwards.

