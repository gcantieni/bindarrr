# Bindarrr

This is the website that hosts the contents of the nautical songs and shanties in the bindarrr for Brown's *only* pirate a Capella group, ARRR!!!. 

## Structure

Songs are  stored in `songs` and the tools and files for auto-generating the website are  under `build`. The main web pages are stored directly in the root directory.

## How to update

### Convert Google Doc to markdown
To update the contents of the website, you'll want to clone this repo. After modifying the Google Doc to the state you want it in, you can go to `tools > script editor`. Then copy and paste the contents of `build/convert-google-doc.gss` into the editor page (overwriting what was already there) and use `run > convertToMd`. This should send a `.md` file to your email. You can save this as `build/bindarrr.md` (replacing the old `bindarrr.md`).

### Generate the site based on the new markdown bindarrr
Now you can run the site generation script. It has a few dependencies. Install any that you don't have: `pandoc`, `bash`, `awk`, `sed`. 
```
./build/build-song-page
```
This should take the contents of bindarrr.md and convert all songs into individual webpages under the `songs` directory and update the table of contents in `index.html` to point to them. Hooray, you've just created an updated version of the website! 

### Push the new website onto the web
Now you can just take these  local changes push them to the website:
```
git add .
git commit -m "Updated the bindarrr to 20XX version"
git push origin master
```
