# ACNHC-WEB To-do list

* ~~Connect to database crate~~
* ~~Set and retrieve cookies~~
* ~~Add new user from login page~~
* ~~Retrieve full user record from UUID~~
* ~~Retrieve owned fossils for user~~
* ~~Fossil Edit page~~
* ~~Fossil report logic~~
* ~~Fossil all users report logic~~
* Fossil 'whogot' report logic
* Fossil 'whoneed' report logic
* Alias changing
* Logout/cookie clear
* Auto-login through bookmarked link
* ~~Process fossil editing form to save changes to DB~~
* ~~Fossil edit template~~
* Fossil report template
* Fossil all-report template
* Fossil whogot/whoneed template
* Make sure logins work through login page
* Auto version in navbar

## Easy stuff once the above is done (lots of copypasta)

* Retrieve owned recipes for user
* Recipe Edit page
* Recipe report logic
* Recipe all users report logic
* Recipe 'whogot' report logic 
* Recipe 'whoneed' report logic
* Process Recipe editing form to save changes to DB
* Recipe edit template
* Recipe report template
* Recipe all-report template
* Recipe whogot/whoneed template
* Retrieve owned Artwork for user
* Artwork Edit page
* Artwork report logic
* Artwork all users report logic
* Artwork 'whogot' report logic
* Artwork 'whoneed' report logic
* Process Artwork editing form to save changes to DB
* Artwork edit template
* Artwork report template
* Artwork all-report template
* Artwork whogot/whoneed template

## Finishing touches

* Genericize all the routes and functions so the item type is taken directly from the URI itself
* Genericize the Context structs
* Possibly do some genericizing on the DB side
* Maybe make the items have a trait or something so we can use "Where" clauses?
* Universal catch to redirect to login if the userid cookie isn't set properly or the user can't be found in the database
* Something to import the old fossilmajig database
* Post-form-processing notifications through Rocket::Flash?

