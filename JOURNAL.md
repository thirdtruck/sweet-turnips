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

Hmm. `config` seemed like a promising place to start but it goes deeper into trait management than I want to tackle right now (here just past 10pm). I think I need to listen to my gut and try making a _different_ game and let that guide my refactoring decisions.

Okay, let's commit to having a _fixed_ sprite set. I can at least extract that part.

There. Moved a good chunk of `ggez` stuff out of `src/main.rs` and into the `sweet-turnips` crate. That's more than enough for now, both because I meant to be asleep by this time and because I _really do_ need that second proto-game before I start refactoring for the sake of refactoring.

I can always implement these proto-games, including `bitter-turnips` itself, as `/examples`! Yeah, let's flip the script here and move `src/main.rs` into `examples/bitter-turnips.rs` or the like.

Alright. Finally used a trait all my own. Also, I wonder how long the villager spawn event has been broken such that it ignores the cursor position ...

# 08May2020

Dabbling in refactors to divorce `bitter-turnips` from `ggez` in the hopes of (a) reducing compilation time with pre-compilation of `sweet-turnips` and (b) make it easier to replace `ggez` later if I can't fix the sprite rotation issues otherwise.

Doesn't seem to have had a noticeable effect. I'll probably have to shift more things into runtime dependencies, e.g. the config file.

Let's put that `Cursor` where it belongs instead.

So that refactoring wasn't too bad. That _did_ end up tying the cursor position update to the world advancement rate, though, so I'll have to fix that.

Fix wasn't too bad! I realized that I could _resolve_ any outstanding events in the `World` on every `update` call even if I don't `tick` the internal world clock over. That let me preserve `CursorMoved` as a game event like any other.

And removed some unnecessary complexity from the rendering code that was bugging me!

# 10May2020

Time to make `bitter-turnips` an example and `sweet-turnips` the main module! Well, that was super easy! Time to make _another_ game using this library, then.

**Damn!** Two hours or so later and I have a proof of concept for a shmup (shoot'em up)! Simulated scrolling and player ship movement already! Feeling high hopes for this proto-game approach.

# 13May2020

Time to play around with my brand new nanoKONTROL2 MIDI controller! Let's see how readily I can use it as example input.

It worked! Now you can steer the ship and even control the flight speed (well, the world tick speed) via the MIDI controller! I'm mad (elated) with power!

# 16May2020

Time for a little refactoring! Let's extract that MIDI code out into `sweet-turnips`.

And that was pretty straightforward! Rust and type-checking for the win.

Added an enemy ship and made it move! Yay!

Now let's refactor a bit. I've been bitten before by neglecting to assign an entity to all secondary maps it would need so I want to move that initialization into the initializer function for each entity struct... Actually, I'll take that back. I don't like the idea of the entities knowing too much about how the `World` treats them. Gut instinct.

Alright, time to make the enemy ships actually dangerous! Well, that took a bit more trouble than I expected but now it works! (Next: Fix the code where it assumes there's only one player ship since there can now be zero ships. Update: That wasn't too hard to fix at all!)

# 20May2020

Felt like squeezing in a little dev time before bed.

Ran into a curious error while adding the "enemies disappear off the bottom of the screen" functionality. I hit a state where the game tried to update the coordinates of the enemy after all of the enemy's ECS mappings had been cleared out. I fixed it but it required more knowledge of the system than I liked. Perhaps I need to add certian rules to enforce this, like removing more mutability and enforcing a more functional style.

# 24May2020

Time to dip my toe into Rust macros! Been meaning to try that to make sprite helper method management less painful.

Rethinking this macro idea. I can probably get more mileage out of using more general methods.

Okay, refactoring towards general methods seems to have been the simpler approach. No need to use macros unless I actually need them. Might come up later if I end up craving for a sprite-defining DSL.

That said, I still don't like how sprite modifiers are handled right now. Let's see if we can improve on that.

Phew! That was a lot of refactoring but I like how handling feels now. More ergonomic.
