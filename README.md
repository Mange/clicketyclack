# clicketyclack

> For that extra clicky feeling inside your headset without waking up your
> family in the night.

This was originally written as a joke, but I found that I actually enjoy it at
times unironically. This program runs in the background and plays mechanical
keyboard clicking sounds whenever you press a key on your keyboard.

Are you using silent switches because you don't want to bother other people,
but wish you could be sociopathic and be able to use the most clicky of
switches? Why choose? You can run this to have the same sounds inside your
headset while you type while the other people around you don't have to suffer.

## Platform support

* **Supported right now:**
  * Linux/X11
* **Planned support:** ("Some day", like that will ever happen…)
  * Windows
* **PRs welcome for:**
  * macOS
  * Windows
  * Linux/Wayland
  * Other platforms

## Switches

* Kailh Box White

## Q&A

**Surely this is a joke?**

Well, yeah. Didn't you read the opening paragraph?

**Can the volume be changed?**

Yes, you can set a base volume with the `--volume` (or `-l` as in "level")
argument, which will be fixed for the duration of the program. `1.0` is
unchanged, `0.5` is half volume, and `2.0` is crazy.

If you want to change the volume at runtime I recommend you use your OS audio
mixer. In Linux, for example, the playback will show up under PulseAudio as a
separate stream and you can boost or reduce the volume as you wish there.

**Are more switch sounds planned?**

Maybe. I have a few laying around. PRs are also very welcome as long as you can
verify that you recorded the sounds yourself and didn't steal them from
somewhere else.

**How do I record sounds to be able to contribute?**

Don't bottom out the key; that sound will already be made by the real keyboard
if the user bottoms out, or not made if they don't bottom out. It should not be
part of the recording.

Try to cut the audio to right before the sound is emitted to reduce delays
between actual actuation and the sound getting played.

**I'm not able to write code, just record switches. Can I contribute?**

For sure. Send me the sounds somehow (email, PR, etc.) and I'll do the rest for
you.

**How does it work?**

The main application is fairly small with most of the complexity laying inside
adapters for the different platforms. For Linux that adapter is written in C
and uses the (pretty much undocumented X11 API) to record key presses.

When the application is built for a specific platform that platform's adapter
is also compiled and embedded.

## License

Released under the MIT License. See `LICENSE` for the full license text.
Copyright © 2021 Magnus Bergmark
