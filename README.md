# Installation 
`cargo add --git https://github.com/janTatesa/ratatui-minecraft`
# Usage
The library provides a simple `run` method which takes an initial state, a function that refreshes the ui and a function that handles events. After implementing the application simply just `cargo run` and type `localhost` to the IP adress field in minecraft server creation. You will be greeted wwith more instructions
# WARNING
The server is set to offline mode (to make debugging easier) meaning that there is no account verification, and everyone who has access to the port `25565` on your computer can connect to the server and interact with your application. **THIS IS AN EXPERIMENTAL PROJECT, DON'T IMPLEMENT ANY SYSTEM APPLICATIONS WITH IT**
