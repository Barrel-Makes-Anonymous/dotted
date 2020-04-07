# .dotted
###### Simple config management.

## Building
Clone this repo, navigate into it and run `cargo build --release`. The binary will be located inside the repo at `target/release/dotted` You can then put the binary somewhere on your `$PATH` so that you can run it from the command line. You can find cargo [here](https://www.rust-lang.org/tools/install) or through your package manager. I will eventually put up binaries when I consider this project "stable."
## Usage
The point of dotted is to create packages which hold files and store where those files should be on the system. You can then enable these packages by either symlinking or copying them to their destinations. Packages can contain files that are not tracked by dotted, so you can easily use git or other tools to manage their contents. Dotted's format is also compatible with other methods of dotfile management. For example: a bare git repository can be used by including a .dotted file and filling out its contents appropriately.
#### Options
* `dotted -a <files> [-d <destinations>] -p <packages>`
    * add files listed to packages listed. If no destinations are specified, dotted will record the original location of each file.
* `dotted -r <files> -p <packages>` 
    * remove files listed in packages listed
* `dotted -m <files> -d <destinations> -p <packages>` 
    * move each file listed in the packages listed to the new destinations listed.
* `dotted -es <packages>`
    * enable the packages listed via symlinking
* `dotted -ec <packages>`
    * enable the packages listed via copying
* `dotted -E <packages>`
    * disable the packages listed
* `dotted -R <packages>`
    * disable and then delete the packages listed
* `dotted -i <files>`
    * move the packages located at the paths listed into the local data dir (`~/.local/share/dotted` on Linux) so that they can be referred to by name instead of by their full path.
    
#### Example Usage
   Here is an example scenario in which dotted could be used:
   
   `dotted -i ~/Downloads/my_dotfiles`
   
   Installs the package, "my_dotfiles."
   
   `dotted -a ~/.vimrc ~/.vim ~/.config/nvim/init.vim -p my_dotfiles`
   
   Adds some vim configuration files into "my_dotfiles." Packages are created when files are added to a package that does not already exist.
   
   `dotted -es my_dotfiles`
   
   Enables the package, "my_dotfiles" with symlinking. If the destinations of files within the package already exist, dotted will ask for permission before overwriting them. If I decided that I want to try out some other vim configs, I can safely disable my current ones without worrying about deleting them for good using
   
   `dotted -E my_dotfiles`
   
   This will remove any of the files that came from the "my_dotfiles" package without touching anything else.
