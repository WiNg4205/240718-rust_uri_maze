# 18/07/24 Rust Uri Maze (#3)
This is a very basic web server where the goal is for the user to click on
options of letter buttons or a back button to eventually reveal a kind message!
The current path of the message is shown at all times as a HTML heading. The
message is finished when the '!' character is clicked.

My god I forgot how painful it was to code in rust. Originally I thought this
was going to take a few hours and could've counted as a daily project.
Unfortunately it has taken me three days to finish this one. Towards the end I
dedinitely lost some discipline and started using a lot of unwraps and
inefficient structure to account for the edge cases. In other news, I hear
the CrowdStrike outage was caused by some memory issue so maybe they should
consider using Rust instead of C++ moving forward 😂.

This was my first time setting up a web server in rust from scratch so I did
learn what information is transmitted from client to server and vice versa at
the application layer and how to implement this in rust. I also had a nice
brush up of my rust fundamentals by implementing trees, string manipulation and
control flow.
