# Bindarrr

This is the website that hosts the contents of the nautical songs and shanties in the bindarrr for Brown's *only* pirate a Capella group, ARRR!!!. 

## Structure

Songs are  stored in `songs` and the tools and files for auto-generating the website are  under `build`. The main web pages are stored directly in the root directory.

## How to update

### Save google doc in markdown format

Google Docs now has native markdown export. Simply go to `file > save > markdown`. Copy or save the file to `md-converter/bindarrr.md` (replacing the previous one).

### Generate the site based on the new markdown bindarrr
Now you can run the site generation program. It is a rust program, which is the only dependency. Install rust if you don't have it, and run `cargo run` in the md-converter subdirectory. You can set `DEBUG=true` in the environment to see more details.

This will output to `md-converter/output` and will contain both an `index.html` file and a `songs` directory of html pages. You might need to slightly hand-edit the md file and re-run if Google has decided to output some nonesense. Once you're satisfied, you can copy a back up of the current songs directory into e.g. `songs-2026`. Then you can overwrite `index.html` and `songs` to contain the versions from the `output` directory. You can then safely delete the `output` directory, it's just for temporary use.

Hooray, you've just created an updated version of the website! 

### Push the new website onto the web
Now you can just take these  local changes push them to the website:
```
git add .
git commit -m "Updated the bindarrr to 20XX version"
git push origin master
```
