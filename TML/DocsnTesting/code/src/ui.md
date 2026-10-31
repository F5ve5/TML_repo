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

----

It seems that there might be a chance of my coding time not being received by the server as i am coding on this device for some reason.
I'm going to just a type a bit randomly because I think there is no way that all I've done yesterday has resulted to less than
4 additional percent. Still, the bar hasn't moved in the oh now it did now it shows two minutes. Let me code for a little bit more and
then refresh the website which is tracking my coding time. It'd be a problem if it isn't because then I wouldn't be able to finish it in
my current location without bothering to set all of that hackatime shit up again and maybe not even then perhaps because this is like a
school laptop or something. It's still only two minutes like what? This is a really big block of text I feel like oh 4min but I feel like
it should've reached more by now.. Furthermore I think that that is one cause for the lack of coding time 6min and the other might be
that time might not progress before you write the 4h devlog which would be a real issue since I just ignored that point and kept coding.

Anyways I think I will keep up on this device and hope for the best. I also need a demo-link to show off basically just source code so
that's a real pain in the ass too.

Hopefully there's just like last week a period of 12h in the start where I'll be able to finish what's left otherways I'm kinda cooked.

There was also like another sort of selection within terra that let me add another "hackatime project" and I added TML_repo onto the
existing TML and I don't recall the former being there on my home device. Hackatime seems so unnecessarily messy.

Woah the Terra progress bar just jumped by 11% which'd be 66min.. I'm at 20min rn.

----

So after the green flag is received the flow should switch from a loading screen onto the same underlying loop that'll be running for the
rest of the program, that also means that the channel calls will start at that point and not before. The main thing of the whole idea is
obviously the timeline so why don't I spend the next five hours working on that before shipping a demo of it.

---

Now the thing is that I thought that because egui/eframe is "dynamically generated ui" or something like that it'd be as or more optimal 
to generate the entirety of a graph each frame than modifying the existing one according to the changes since its' last call, that
theory was incorrect so I'll have to remake the payload channel convention to fit the latter.

Now the type of rules that this data would adhere to is completely unique compared to the rest of its' like, as it is reset upon each new
call for it itself or the data that it represents. One way that I could make this would be to change the UiLoad structure to contain
additional variables which represent the change in larger datasets like btreemaps and I could have these variables adhere to different
rules than the base ones....

So instead of the preprocessing uiload simply being cloned to the postprocessing one it could be "added" and then each time that postproc
receives a call towards this special delta variable, it resets it. So that it may only ever represent the change since ui last looked.

Since if the preprocessed data is added rather than cloned then no already accounted for data is shown again

I think the best way to evaluate a change like this is naturally in the ui_preproc part as it is the part to receive the changes
themselves as opposed to if it were handled in the ui_postproc part where only the existing timeline or whatever is available so the
changes would have to be like evaluated and stored when they are readily available in the former

-

Having made the system for acquiring the changes in the timeline rather than the entire timeline upon update, I'm starting to really
reconsider the fact that the ever-increasing BTreeMaps' are cloned every single time events are processed which is basically all the time

A much better approach would be to merge only the lighter variables necessary to generically update the ui perpetually, such as these
"change" ones

That although leaves the question of how and when the heavier variables are to be merged. Perchance upon request? Obviously from the ui
thread but to the pre- or postprocessing thread? All other requests are made to the postprocessing thread but it'd not be in posession
of said data in the given framework..

Acutally I think the cleanest approach would be to establish another channel with one sender in the postprocessing thread on the
receiver in the preprocessing thread so that requests are still made through the same mechanism from the ui thread and exceptions are
handled when they appear by the backend rather than me having to consider what needs what when coding the ui thread

Another sender for the payload channel would then also be established in the preprocessing thread

-

                let mut pid_v: Vec<u32> = vec![];
                for v in rup.events_by_uid.iter(){
                    let mut closest_start: u64 = 0;                
                    for e in v.iter().enumerate(){
                        let t = e.1.timestamp.unwrap();
                        let o = e.1.opcode.unwrap();
                        if o == 1{
                            if t > closest_start && t <= opcode.xII{
                                closest_start = t;
                            }
                        }
                    }
                    for e in v{
                        let t = e.timestamp.unwrap();
                        let o = e.opcode.unwrap();
                        if o == 2{
                            if t >= closest_start{
                                if t < opcode.xII{
                                    continue;
                                }else{
                                    pid_v.push(e.process_id.unwrap_or(0));
                                }
                            }
                        }
                    }
                }