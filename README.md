# Lyoko frames

a daily guessing game for Code Lyoko fans highly inspired by [framed.wtf](framed.wtf).

## Idea

Every day, a random episode of the Code Lyoko cartoon is randomly picked ; the player will be shown up to 6 different frames from the episode, and will have as much tries to guess which episode do the frames come from.  
The player will start being shown only one frame, and if the try is a fail, then the web page will display the second frame and allow him for one more try, and so on until the 6th frame. If the 6th guess is wrong, the player loses !  
The frames are sorted by how much they help finding the episode : the first frame will not give much information and make it hard for the player to find the good answer that early ; the 6th frame will most likely show a memorable moment of the episode.

## Conception

I am currently writing this project on my free time. This section is a global idea of how I imagine the website to work once done, and I can use it as a guideline to write the server code.

The website will be based on one unique web server, written in Rust with the warp library.  
The only two web pages (for now) are /index.html and /game.html. index.html explains the game to the player, and contains a link to game.html, where the player can then play the game.  

The /game.html page should first show one frame to the player, and give him a list of all episodes (with episode number and name) to choose from. The frame will simply be a `<img>`, and its src is gonna be `/frame/<frame number>`. The webserver will serve the right image depending on `<frame number>`. Note that this approach implies that the time the day is "over" and the episode changes depends on the server's time ; it will probably be changed at 00:00 UTC.  
If the guess is wrong, a message to indicate the answer is wrong should be prompted, and the frame should be replaced by the second frame. The first frame must also still be visible by the player. Buttons to see the already discovered frames (just like in framed.wtf) would probably be a good user interface.  
If the player's guess was good, a message indicating that it is the good answer should be prompted. This message should be quite big and visible.  
The `/frame/<frame number>` route to get a frame makes it easy for anyone to just make a request to this route to see a frame they should not be able to see yet. I am very aware of it, but I do not wish to put too much effort on this project, and will let the player do what they want. If they want to cheat, they miss the whole point of the game and I don't care.  
To make this cheat method impossible, a solution would be to give each player a session cookie, and verify based on the previous requests made by a user identified by its cookie if they are allowed to see the frame they request. I am a Rust beginner though, and I don't want to implement such a thing that would require me too many effort :yawn:.

## Frames choosing

One of the part that is gonna take some time is to actually choose which frames to take for each episode. I recently rewatched the whole cartoon, but I didn't have this project in mind at that time. Now I am gonna have to go through each episode and take some interesting frames, that should be balanced for the player so that they can find the answer not too easily nor hardly. Wish me luck :D

## Contact

romain74.baudet@gmail.com