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
