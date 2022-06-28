# monthly-code-challenges

This repository will consist of our monthly code challenges. This is what that entails:

- A new challenge is released the first week of every month
- The challenge would be something fairly simple and shouldn't take a very large amount of time to complete or detract from our regular responsibilities; the monthly aspect is meant to give people ample time to work through it
- I'd likely pull challenge questions from various sources (e.g., Google's Code Jam archives)
- We can discuss and review solutions in a separate teams chat meant for this, maybe also using a repo on bitbucket so we can view everyone's solutions
- This would be optional participation of course, if you don't want to or can't that's entirely fine
- THE TWIST: we must complete the challenges using a randomly selected programming language for that month

To participate:

- Fork this repository
- Check out a month's branch
- Use VSCode + the devcontainer definition to get working in an environment for the language of choice
- Push your solution to a branch on your fork and open a pull request against this repo's branch

-------

# July 2022 Challenge

For this month's challenge, we will write very basic PUBSUB server-client programs using [Rust](https://www.rust-lang.org/)

The goal of this challenge is to learn how to set up a TCP server that receives messages from "sender" clients, and forward those messages to all "listener" clients.

There will be two programs that you will write, one for the server and one for the client.

## Server Specifications

* Accepts TCP-based connections on some arbitrary port(s)
* When a "sender" client sends the server a message, it will forward it to all registered "listener" clients

## Client Specifications

* You should be able to run multiple instances of the client program at a time
* A client instance should run in either "listen" mode or "send" mode
* "Listener" clients will print all messages it receives from the server
* "Sender" clients will allow for user input 

## Additional Design Considerations

* You'll need to consider how to delineate messages, you're free to pick whatever method you so choose
* You'll need to determine how you want to specify to the server whether a connecting client is a listener or a sender. Some options might include: (1) listening on two different ports, one for each client type, (2) an initial message on connection to differentiate the messages
* Consider how you want to tell sender clients what messages to send. E.g., user input accepted during run time, stdin/file based, etc.,
* If you find it easier, you may also split "listener" and "sender" client programs into their own cargo projects with their own main files rather than having a single program that runs in two modes.

## Bonuses

The main challenge was designed with minimal specs to keep things as simple as possible, since the learning curve of Rust can be a bit high. If you find yourself with additional time, feel free to have fun with this and extend the functionality as much or as little as you like. Some possible extensions might be:

* Allow a single client instance to be both a sender and receiver
* Have message "topics" that some listeners might subscribe to and not others; only messages for that topic get sent to clients listening on that topic