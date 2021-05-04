## Shortcomings

This project is not designed for anything nearly anywhere approaching real-world use.
The purpose is purely for learning DB skills. As such, the following shortcomings are present:

- The Passwords are stored entirely in Plain text
- The application is entirely synchronous, meaning the application will freeze while waiting on longer DB queries/functions
- All monetary data in the DB is stored as floating point types instead of SQL's `NUMERIC` type. This is done because 
rust has no equivalent type to `NUMERIC` and as such the create being used for SQL connections could not handle this.
- The DB Connection details are hard coded into the application, and have no provision for an external server. 
