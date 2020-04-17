# ACNH Catalog

An attempt to make an extensible cataloging system for Animal Crossing: New 
Horizons.  Make a user, track your fossils (and recipes and possibly more), 
see who's got what you need, and trade!

It's meant to be instanced per community; ie Discord servers with a lot of 
people who play the game and want a nice centralized way to share their 
extra items.

Currently under a big rewrite with the database portion and web portions 
separated into different crates.  Database is currently focused on Sqlite3 
via Diesel, and Web is using Rocket (and therefore rust-nightly).

