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
