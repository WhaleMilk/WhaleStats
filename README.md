# Golden Stats

A statistical analysis tool for League of Legends which allows for a greater, more in-depth analysis of your performance across games. Locally hosted, GoldenStats provides visualization of how your performance has changed over your recent games, and further detailed breakdowns of stats and interpretations of said stats are planned features. 

Written entirely in Rust with Tauri, the program asks the user for their Summoner Name, tag, and server, and then fetches relevant data from the Riot Api and processes it with the analyzer_core module. This then is passed to the front end where relevant charts are built. 

## Tech Stack
- Rust
- Tauri
- Microsoft Edge Web Viewer
- Apache E-charts

## Features

Currently, GoldenStats is incredibly barebones and only provides graphs for four statistics. Further features such as game-specific breakdowns, graph interactivity, user comparison, and team analysis are all planned. 

## Learning Process

This project was how I decided I was going to learn Rust. Rust was a language I was very interested in, and so after finding Tauri, I decided I was going to fully dive into the language and figure out how I was going to make this app, one way or another. I had already written the code that did the backend stuff for a CLI, which is what this project started as, but that was originall written in Go, and my implementation of the CLI in Go was really messy, so I figured I could do it much better. 

The biggest hangup for me on this project was actually the backend. Desigining a good API for the front end to interface with was really tricky. My first implementation of it was a little too granular and didn't take into account future feature expansion, and so I ended up with a bunch of functions which did the same thing but in different ways. So one of the biggest projects in this project was refactoring the entire backend. 

The backend rewrite, despite how big of a momentum killer it was, was the biggest learning experience of the entire process because this is the first large-scale project I've written, and as such was the first time I really had to refactor something big. It was the first time I had to really think about designing something like an API from scratch all on my own. 

## How to Build

Prerequisites:
- Rust
- Tauri
- Microsoft Edge Web Viewer

Steps:
- Clone the repository
- Create a .env file in the repository root
- Go to [this page](https://developer.riotgames.com), sign in or create and account, and generate an API key
- past api key into the .env with format `API_TOKEN = {token}`
- run with command `cargo tauri dev`
