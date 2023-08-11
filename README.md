# Logineo-scraper
A script to scrape personal information from logineo instances.

## Explaination
The great state north rhine westphalia created a _great_ "school management system" which also
contains an email frontend which has a great auto completion when entering the receiver requiring to enter 2 characters which shows all users starting with those characters. By doing a little automation you can scrape all user information, which is defined [here](https://github.com/iraizo/logineo-scraper/blob/master/src/main.rs#L6).

## Usage
Before starting you should be having some awareness of how the developer tools in your Browser works, especially the Network tab, if not you should just google the stuff where you are stuck at.

1. Open the network tab
2. Clear all the requests
3. Try sending an email to a random recipent like "aa". 
3. Open the request that pops up (the url will contain something along the lines of `search=aa`.
4. Copy the `Cookie` request header.
5. Insert into the quotation marks [here](https://github.com/iraizo/logineo-scraper/blob/master/src/main.rs#L54)
6. Insert the url of the request into [here](https://github.com/iraizo/logineo-scraper/blob/master/src/main.rs#L53) where you replace `aa` with `{}`
7. run `cargo run` inside the folder after installing [rust](https://rustup.rs/)
8. ???
9. Profit! (open output.json)
