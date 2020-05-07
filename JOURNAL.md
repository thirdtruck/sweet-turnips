# 04May2020

Better late than never! Wishing I'd started this from minute one of the game jam but there's still plenty to talk about.

`slotmap` provides support for multiple key types, in order to support stronger type checks between its maps. I'll try adding a `VillagerKey` and see how well that works.

Never mind! All the maps I have right now actually need the same key. I'll revisit this later, like if/when I had a reverse look-up table for coordinates.

Let's try making inline event generation more functional (as opposed to mutable) instead.

That wasn't too bad! At least not the groundwork of refactoring the event resolution methods to return new events instead of modifying the new event stack in place.

Glad I practiced building purely functional event systems in a previous game jam project. The more instances of `mut` we can remove from arguments, the better.

Oops, I started breaking the compiler! Don't ask me how because I don't know.

Well, I _thought_ I broke it but I reverted my changes and it's still breaking. I'm at a loss. Wondering if my new backup software running in the background broke something. Ooooh, right. Disk space issues.

Disk space issues confirmed. Code compiles just fine now. Hopefully that didn't mangle my git repo ... Glad this issue didn't strike during the jam itself! Mind your development environments!

Yeah, moving the example world configuration out of `bitter.rs` and into `main.rs` was long overdue. Wishing I'd made that externally configurable a while ago, too. That would have saved me a lot of time waiting for builds to finish. In fact, let's do that now! It's high time I learned `serde` anyway.

I'm learning more about `serde` and error handling today than I expected to. Might have been for the best that I never got around to setting up a config file within the game jam itself.

Nope! I take it back. The time difference in loading a file vs. compiling is huge! Would have saved me of time _and_ **morale** during the jam. At least now I know how to do it next time!

I _did_ go down a rabbit hole on custom error creation but at least I dug myself back up out of that and found a much simpler solution for now.

So that's one takeaway: Build configurability into your game project early and often! (Unity gives you this out of the box but it's imperative for tools like Rust if you want to reduce how often you rebuild the game, which will be _often_ in a game jam.)

Now let's do refactor this to use `From`! This should make things more idiomatic and easier to read.

What's next? Let's keep making the game state more immutable!

Well! That refactoring is coming along nicely. Dinner time, though, so I have to shift gears for the rest of the night!

# 06May2020

Dabbling in refactoring tonight. Wanted to see how much trouble it would be to extract a library with common functionality, at least on the purely technical part. Wasn't hard at all! I'm hoping that I can speed up build times by moving more code into the crate so that it doesn't have to be recompiled every time.

Time to practice my Rust traits, too.
