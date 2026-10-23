I feel that I finally know of the intricacies of the ui loop of egui/eframe.

I'm not too sure where I came from before but now I understand that it is I that decide how the following frame is to play out rather than egui calling it by itself.

With that, I have also come up with a system with which each frame is to adhere to.

I so far call it the nano-token system.

The flow goes like this every frame:

-----

The time at the start of the frame is stored.

Each frame keeps count of a certain amount of these nano-tokens and the following part consists of spending those tokens on the different tasks that are waiting
to be executed.

For example, say that there are 3 different tasks that have to be done this frame thanks to input, having received a channel message or simply schedule;

The user has clicked an information button (Not really a heavy task to display it, 5 tokens)

A timeline update has arrived and is waiting to be shown (Presumably pretty heavy, 26 tokens)

Something else idk (30 tokens)

Now if the token amount that each frame is given is 50, the first two tasks will be executed and the third one will be appended until the next frame because
the remaining tokens are not enough to afford it.

And as a complementary addon, after the initial execution of all previously appended tasks the remaining otherwise useless tokens can be spent on something like:

while nano_tokens > 0{
    if let Ok(msg) = rx.try_recv(){
        try_do_task(msg);
    }

    nano_tokens -= 1;
}

To execute possible later-received desynced tasks.

At the end of the frame, the time at the start of the frame is compared to the current in order to timely execute the next frame.

-----

This system will allow me to run a fitting amount of wrok every frame without needing constant time lookups which are presumably more expensive.. maybe should've
checked whether that's true by now.

It is;

ctx.input(|i|i.time);

by itself remains the same for the remainder of the frame and an OS lookup is probably necessary to retrieve the actual time.

Instant::now();

however does get the current time and it is according to CoPilot really cheap as well

----

Going through this plan once makes me realise that I also need a way to like take tasks from the rx and store them for later because I can't just leave them in
the rx and only check them, I have to take them out. I'm pretty damn sure that's how it works.

Shouldn't be too difficult though, I'll just arrange them along with the other task request things.

That's another issue. I have to make all manners of requests compatible with this token system.

Another problem is like, what if anything do I want outside of this token-system? Everything takes time but I guess if there's something so crucial it should
be run within every frame.

---

Let's just make up a framework containing all of the theoretical bits and pieces of a single frame loop. I'm pretty sure input is going to be split up into three
distinguishable categories, being what signals tasks to be done:

1. Physical Input - Input from the mouse or keyboard or whatever which is supposed to directly change what is on the screen plus potentially doing something else,
this type of input is going to be completely dependent on the apps current state so a mouseclick on X=15,Y=4 will mean something different depending on what is
currently on the screen.

So before a click is scheduled as a task it has be compared to the current conditions.

I'm thinking that a clean solution to this problem would be to have one 2 dimensional array for each "layer" of the program and some are put further ahead than
others; so when a click is evaluated, an array of 2 dimensional arrays is checked from top to bottom and the first 2D array to present a non-null solution for
the given position is the one to arrange the task in response

2. Channel Input - So not when the ui thread sends a message but when it receives one. Channel input should not be treated as an input type as much as it
primarily serves as a backend info broker.

I'm thinking of storing what will be the great majority of these channel messages inside of a vector where each type of message will occupy an index in it.

And when an info window is opened wanting to display the same info but updated, the previous info in that index is removed if it isn't already and a channel
message is sent to retrieve it anew.

3. Scheduled Input - Now I'm not sure if this needs a category of its' own but they'd work like channel input but just even more disconnected from physical input.

So like not the type of input that arrives on user input request but completely independent of it, but this'd probably also primarily arrive from the thread and
be handled in a the same way as the earlier vector index method I mentioned.

I'm happy with this

---

I'm thinking physical input should be evaluated first

well first of all should probably be the green flag from the other channel

so nothing else is done until that green flag is gotten, perhaps additional processing from that green flag thread for like a more informational loading screen

like loading files... loading something else...

some sort of progress bar

Meh that would be kinda messy actually, let's just have like a loading animation before it starts

    let mut tries = 0;
    loop {
        match TX0.get().unwrap().send(bin_vec) {
            Ok(_) => {println!("yayyyyyyyyyyyyyyyyyyyyyy, {:?}", tries); break;},
            Err(v) => {
                bin_vec = v.0;
                tries += 1;
            }
        }
        if tries > 10_000{
            panic!("Its so overrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr");
        }
    }