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

# Jun 2022 Challenge

For this month's challenge, we will write a simple Wordle clone using [ReSript](https://rescript-lang.org/) and [Faker.js](https://fakerjs.dev/guide/)

ReScript is a compile-to-JS language based on the OCAML functional programming language. Historyically it was a compiler for the ReasonML language but the folks behind the compiler opted to create their own variant of that syntax to make it more friendly for JS developers. It has a sound type system and has no null/undefined errors due to its explicit handling of nullability and exhaustive case checking.

One of its most powerful features is its [pattern matching](https://rescript-lang.org/docs/manual/latest/pattern-matching-destructuring). This style of programming is concise and makes exhaustive case handling simple. I'd recommend you try to make use of it to some degree to get a feel for it.

Another neat thing about ReScript is that the compiler output js files are usually very human-readable and look almost hand-written in nature. I've opted not to mark the .bs.js generated files as part of .gitignore so that we can see the output when reviewing the solutions. This is actually one of the recommended approaches by the ReScript team as a way to get new devs used to understanding how the rescript code looks and make code review easier to get started with.

Fun fact, React was first protyped in Standard ML, a functional programming language with many similarities to OCAML/ReasonML/Rescript. You may catch on to some of these concepts that made their way into React as you work on this.

This branch uses [Vite](https://vitejs.dev/) as its build tool. This is similar to WebPack but rather than building full bundles it leverages ESM in the browser to serve files without pre-bundling the code.

To get started run `npm run start`. This starts vite which serves the application and rebuilds/refreshes when changes are made.

Something you'll notice here is that the rescript compiler will generate .bs.js files

## Game Specifications

* The user has six guesses to match the word (generated by Faker.js)
* Each word is five characters
* A correct letter turns green
* A correct letter in the wrong place turns yellow
* An incorrect letter turns gray
* Letters may be used more than once

For fun, feel free to tweak these parameters if you'd like (e.g., maybe 7 letter words, 4 chances instead of 6, etc.,)