# Copy-pst
A simple cli tool to copy and paste files/directories.

# Install
Compile (`cargo build --release`) and either sym link or move the binaries in `/target/release/<pst/copy>` into somewhere in your path

# Usage
The copy tool copies the absolute path of the directory/file listed into your clipboard and the pst command then runs `cp` for you to paste the directory in a location
```
copy target/
cd ~/Documents
pst ./
```
