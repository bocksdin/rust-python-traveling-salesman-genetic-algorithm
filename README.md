# Rust vs Python: Speed and Optimization

### Introduction
Hello! Welcome back to another video. 
I wrote this traveling salesman genetic algorithm in Python a few weeks ago.
Now, this example is very simple; only 20 routes, and it was taking 5 seconds to run.
Imagine how long it would take to calculate the route from Washington to Florida.
I figured there is no way this method is viable, but I've always had an interest in learning Rust,
so I decided to learn Rust by translating this program from Python.
This was my first experience writing a program with a low-level language, and I was astounded when it ran in only 400ms!
That's about 12 times faster than its Python counterpart, and it happened to get a viable solution more frequently.
After some research and further optimizations, I decreased the Python version to about 2.5 seconds, and the Rust version to about 70ms!
I'm certain there are more optimizations to be made, but I'm pleased.
Let's take a look at what the algorithms are doing, and where I was able to find performance increases.

### Python walkthrough
For anyone not familiar with the traveling salesman problem:
you've got a list of cities, and you want to calculate the optimal route
that visits each city exactly once.
For this example, we have 5 cities, 20 paths in total, and optimal solutions are either the first 5 paths or second 5 paths.
The optimal routes have a total travel time of 0.83 hours.
The paths have an origin city, destination city, distance, average speed, and travel time.
Each traveler is a combination of paths and scored according to the paths chosen.
The algorithm utilizes the uniform crossover method, and has a mutation rate of 1 percent.
The fitness function has an early break condition on the amount of paths chosen is more than or less than the number of cities.
Otherwise, calculate the total travel time of the route and verify each origin and destination city is visited exactly once.
The algorithm otherwise functions exactly as the one in my "Genetic Algorithms: What they are and how to build one" video, linked in the top right of this video.
We're going to run this 50 times, with 1000 generations, a population size of 200,
and we'll print the best solution found, the average solution found, the average run time, and the total run time.
I'll see you when this finishes.
(review results)
Now let's look at the Rust version.

### Rust walkthrough
Again, we have 5 cities, 20 paths in total, and optimal solutions are either the first 5 paths or second 5 paths.
Optimal routes have a total travel time of 0.83 hours.
Paths have origin city, destination city, distance, average speed, and travel time.
Travelers utilize uniform crossover, a mutation rate of 1 percent, and the fitness function is the same.
We'll run the algorithm 50 times, with 1000 generations, and a population size of 200.
We'll also print the best solution found, the average solution found, the average run time, and the total run time.
(review results)
On average, the Rust implementation is about 34 times faster than the Python implementation.

### Optimizations
So, Rust being a low-level programming language, and Python being about the highest of the high-level programming languages,
Rust was basically destined to beat the Python implementation.
As I stated earlier, my Python program was running about 5 seconds, and my Rust program was running about 400 ms.
So what optimizations did I identify that brought them down to 2.5 seconds and 70 ms, respectively?
For starters, on the Rust side I switched from using dynamic length vectors to fixed length arrays.
This saved about 50 ms.
Next I stopped cloning arrays. Initially I was storing a copy of the paths array on each of the travelers and the genetic algorithm itself.
By simply passing the paths array around for each use case, this saved about 100 ms, both in Rust and in Python.
In Rust, I switched the crossover function from returning an array of travelers, to returning a tuple of travelers.
This actually saved about 10-20 ms.
Lastly, and by far the biggest gain made, I added this early break case to the fitness function.
In Python this cut the run time in half. In Rust this cut the run time by 75 percent.
This exemplifies the importance of preventing unnecessary work.

### Closing
Performing this exercise has taught me that not only do I have to make sure my logic is designed efficiently,
but also that I need to be cognizant of memory access and manipulation operations.
The early break case may have been the single largest performance booster,
but the memory access and manipulation operations really added up.
I'd say if you're experienced with high-level programming languages,
but not so much low-level programming languages, take a crack at one.
Understanding how the underlying system works just might make you a better programmer.

### Thank you
If you found this video helpful or interesting, remember to leave a like and subscribe.
Thank you for watching, and I'll see you in the next one.