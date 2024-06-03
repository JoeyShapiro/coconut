[fancy mouse stuffs](https://stackoverflow.com/questions/3087975/how-to-change-the-cursor-into-a-hand-when-a-user-hovers-over-a-list-item)
https://github.com/RamiHg/soundio-rs/blob/master/examples/sine/main.rs
https://github.com/RustAudio/cpal/blob/master/examples/feedback.rs
https://github.com/tauri-apps/tauri/blob/dev/examples/state/main.rs#L32

set amplifier in front end
compute amp in front end
get users in back
send users to front end
change users pos in front end
send amp to back end
ok and sounds good. but not scalable for plans
confusing

just have users in back, send list to front end
update with changes
backend needs *will* more than just amp
front end can compute stuff. might need compute on backend
doing all on vbackend would be faster

yeah. should compute on backend. otherwise pass amp to back and users to front. and keep doing this
just do it all on back end. then some stuff on front

i can pass the current user, pass the index, or check for them

packet models
```
version | type | data
version | sound | user | sample
```

the problem is i need a stream and events
i could have an api for getting events, but what if the data (stream) and events are slightly misaligned
stream will be faster
i cant have lag or disabled
so they need to be in the same transfer
requests are too slow and verbose
sockets are too... yeah

i could take a note from midi, and prolly other formats
they do something similar to what i want
data stream is inferred, this will reduces the chatter; and mark an event when it happens

data example
```
version | event | data || version | stream | user 1 | sample | user 2 | sample | user 1 | sample | user 2 | sample
```

but its 2 way. one thing at a time. all i need rn is the stream
```
ex for 2 users
user 1 | sample | user 2 | sample | ...
u8     | f32    | u8     | f32    | ...
```
u8 is kinda big, but i need this alternating pattern
it will repeat this cycle
i can account for failures later. but if the user has no data, it is just 0 for sample
oh wait, what about udp. i need that. this just got hard

u32 1 bit per user?
what?
actually, if i send every user, i dont need id
but if i do dynamic, i do; right right right
could i do bit stuffing.
u32 is u1 u2 u3 u4. padded with 0s
then next n chunks are that user. compact, maybe, but confusing
actually, i dont think this would save much room. will kinda, but not really
i think i just have to make sure im not cross packets


if a user tx, then send it right away. this would get heavy, and how would i merge samples
i can have ther server send batches to everyone. oh and that would handle emptiness
i can send 0s if the user doesnt tx, but i dont think i need to. i already have it set up with user ids
i can send data at a constant interval. this is good testing for multiplayer
rather than every time something happens. that would be a lot of transfering
i will also have packets do multiple samples, as they do not right now
it would be easy to set up, b ut would take WAY too long. that would mean i could compact a lot of the code
and it would be less messy
im still curious about when i should transmit. every message might work in most cases, but lets try this to see
