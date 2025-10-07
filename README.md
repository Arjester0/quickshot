# Quickshot 
a CLI tool to make tmux project sessions come quickly, written in Rust  

To use, clone the project using 
```bash
git clone https://github.com/Arjester0/quickshot
cd quickshot
```
and then run 
```bash
cargo install --path .
```
Once installed and you have cargo added to your PATH in your shell use
``` bash
quickshot
```
to run! 

I high suggest you alias quickshot to qs in your shell config. 

On prompt it will ask you to enter a PATH for your project directory, make sure to type it EXACTLY how it is 
in the prompt, just like this:
```
"/home/YOUR_USERNAME/YOUR_PROJECT_DIR/"
```
 If any errors occur, or you want to change your
project directory go into quickshot.config and edit the dir variable to be like shown here.

Once you are in type the name of the project directory you wish to open a tmux session in and press enter. 
If you wish to escape use ESC twice. 

TODO: 
- [ ] Optimize for performance
- [ ] Allow for editor in separate tmux window 
- [ ] More customizations in quickshot.config 
