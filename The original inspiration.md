I had an inspiration while showering this morning. @Stevan, I know what I'd like to do with your large CNC machine.

First, we need a way of comparing two images and getting a "similarity score" where 1.0 is "the same image" and 0.0 is "two completely different images." Then, we have a user (us, or maybe someone at a photobooth) input a picture of a person's head.

Now that we have a source to compare to, we create an image buffer and draw short, random line segments (or any other sort of random thing) and each time we draw, we check to see if the similarity score went up or went do. We continue randomly placing lines (or possibly preferring precomputed areas of high detail in the original) until we reach some pre-defined "good enough" score.

Now we have a sketched version of the original which we can use to create a life size portrait of the subject. Additionally, we could do this with a color photo if we did the CYMK thing that one guy is doing and combined that with 4 filtered versions of the original image.

What do you think?
