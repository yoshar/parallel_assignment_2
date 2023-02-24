# COP 4520 Assignment 2
## How to Run
cargo run
## Problem 1 Birthday Party Problem
## Problem 2 Vase Problem
### Strategy Discussion
1. This is equivalent to a normal lock on a variable, while one thread (party guest) is accessing the variable (viewing the vase) it does not allow any other thread to access that variable. When another thread would need that reasource they are forced to wait until the other thread releases it, however there is no queue system so it is "random" which thread will get the reasource next. 
    
    The main positive is that this is a simple method of implementing multithreading with a shared reasource. The negative is the next thread to access the reasource is not controlled in any way so there is no way to know which thread/job will completed next or when it will be completed. This can also lead to some cases where there is no gain over single threaded solution.

2. This method is equivalent to a flag solution. This is similar to the above solution but it makes it so that the threads do not wait on the reasource instead they wait for the flag to flip. They are able to other things while waiting for the flag. There is still no method of determining which thread is next or when the next thread will run.

3. The queue method solves the downsides of both of these, this way you know at some point the thread will be able to complete its job. There will be no thread that gets unlucky and is unable to run for a long period of time. The downside is that this method is the most difficult to impliment.

The best solution is either 2 or 3. In the context of the problem option 2 gives the guests freedom to enjoy the party, however there is no way to promise each guest the oppurtunity to see the vase. Option 3 does not allow for them to enjoy the party as much as they would be waiting in line, but they are able to garuntee seeing the vase at some point. I think 2 would be best for the guests as they would be able to other things while waiting on the vase room.

Solution is a thread pool that has a lock for door which is a binary mutex lock. You push however many jobs to the pool then the pool will go through them. I make the viewing thread wait for 1 second to simulate it looking at the vase. Afterwards the lock is flipped back to true ("available") then released when it moves out of scope.

In hindsight the thread pool is not the best as it tends use the same thread for the first few iterations then uses a new one every time after that. I'm not sure why and would have to do more research into thread pools and my implementation of it.