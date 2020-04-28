# ACNHC-WEB To-do list

## Fixes needed after beta test
* ~~All-report shows all users have the same fossils?~~
* * wasn't filtering the owneditems data by both userid and itemid whoops
* ~~Edit forms submit to the same (undefined) url - add a handler for /save/item or make sure the item type is sent to the template for the form action~~
* * Added an itype field to the report context structs for this


* ~~Connect to database crate~~
* ~~Set and retrieve cookies~~
* ~~Add new user from login page~~
* ~~Retrieve full user record from UUID~~
* ~~Retrieve owned fossils for user~~
* ~~Fossil Edit page~~
* ~~Fossil report logic~~
* ~~Fossil all users report logic~~
* ~~Fossil 'whogot' report logic~~
* ~~Fossil 'whoneed' report logic~~
* ~~Alias changing~~
* ~~Logout/cookie clear~~
* ~~Auto-login through bookmarked link~~
* ~~Process fossil editing form to save changes to DB~~
* ~~Fossil edit template~~
* ~~Fossil report template~~
* ~~Fossil all-report template~~
* ~~Fossil whogot/whoneed template~~
* ~~Make sure logins work through login page~~
* Auto version in navbar

## Finishing touches

* ~~Genericize all the routes and functions so the item type is taken directly from the URI itself~~
* ~~Genericize the Context structs~~
* ~~Possibly do some genericizing on the DB side~~
* ~~Maybe make the items have a trait or something so we can use "Where" clauses?~~ <- Can't really do this how I wanted; traits can't be used as function types.
* Universal catch to redirect to login if the userid cookie isn't set properly or the user can't be found in the database
* Something to import the old fossilmajig database
* Post-form-processing notifications through Rocket::Flash?

