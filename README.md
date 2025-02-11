--- TRAINING CODE

A little bit about why this code:
I'm studying Rust, and I like to get my hands dirty. I learn fast by doing it instead of just reading.

So, I've created this simple skeleton of an antivirus, which at first, I thought would be "easy"... but it wasn't. I had to learn about async (which was awesome) and closures.

Now, my idea (it will take a while) is to create a module for the Linux kernel that can consult the syscall table, take a "snapshot", and if there are any changes in the addresses, I can alert through my antivirus.


So what it does?

Very simple: you give it a path, and it monitors the directory. Every time there is a change in that directory, it creates a hash of the file and compares it to the database.
Its just a antivirus signature base.
