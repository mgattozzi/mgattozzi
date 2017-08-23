# Diesel Powered Rocket
Published Jan 9, 2017

This holiday season I had decided to rewrite this website (again) to use
Rocket, a new Rust web framework. I had also
thought, "Why not learn some React as well?". While doing so I ran into
all kinds of learning moments and what I made was basically a static site but it was fun!
While there's much to improve I did come up with a fun
little app that I think can show off what Rocket can do. To set it all up we'll need [Rocket](https://rocket.rs),
[Diesel](http://diesel.rs/), [Bootstrap](http://getbootstrap.com/) and [React](https://facebook.github.io/react/). The primary point of this demo though is to show how to get Rocket to utilize a thread pool of connections to the database with Diesel.

## What does the code do?
It's a very simple page that will display a button, that when clicked
will tell you how many times it has been clicked by users. Pretty simple
but it shows off how to make requests in Rocket and how to use Diesel to
store information.

## React Code
My site has routes to different components that when rendered show up
below the navigation bar. We won't be looking at that code today. Instead
we'll be looking at the component that gets rendered when
we go to [mgattozzi.com/counting](https://mgattozzi.com/counting). This component
is called `Count` and it uses the following code:

```js
import React from 'react';

class Count extends React.Component {

  constructor() {
    super();
    this.state = { count: 0, disabled: false};
    fetch('http://localhost:3000/count')
      .then((response) => response.json())
      .then((responseJson) => {
        this.setState({count: responseJson.count});
      })
      .catch((error) => {
        console.error(error);
      });

    this.clickMe = this.clickMe.bind(this);
  }

  clickMe() {
    this.setState({disabled: true});
    fetch('http://mgattozzi.com/count/', {method: 'PUT'})
      .then((response) => response.json())
      .then((responseJson) => {
        this.setState({count: responseJson.count, disabled: false});
      })
      .catch(console.err);
  }

  render () {
    return(
      <div>
        <div className="panel panel-primary">
          <div className="panel-heading">
            Number of times the button has been clicked
          </div>
          <div className="panel-body text-center">{ this.state.count }</div>
        </div>
        <div>
          <button onClick={this.clickMe}
                  className='btn btn-danger btn-lg center-block'
                  disabled = {this.state.disabled}>
            Click Me!
          </button>
        </div>
      </div>
    );
  }

}

export default Count;
```

Let's start at the top and work our way down. This component lives in the `count.jsx`
file in the repo. With my Webpack configuration it turns all of this into JavaScript
automatically. I find the `jsx` file easier to work with while using React and
I like it a lot in terms of syntax. Plus with Webpack and Babel we get to use all
the new JS syntax goodies! Let's start working through it.

```js
import React from 'react';

class Count extends React.Component {
  // Omitted
}

export default Count;
```

Here we get to use the new import syntax. We're importing `React` from
the `react` package. Webpack figures out how to link this into the code
when it transpiles to JS. In my case it's all the modules installed in
`node_modules` via the Yarn package manager. We then declare a new class
`Count` which is what our component is named. It extends the base
`Component` type from the React library. This means we get all of the
methods and things that `Component` has! Last but not least we export
the class and say it's the only one being exported from the file. While
I could have added the `export default` to the beginning of the class
declaration I added it to the end of the file as a personal stylistic
convention.

Okay let's look into our `constructor` method:

```js
  constructor() {
    super();
    this.state = { count: 0, disabled: false};
    fetch('https://mgattozzi.com/count')
      .then((response) => response.json())
      .then((responseJson) => {
        this.setState({count: responseJson.count});
      })
      .catch(console.err);

    this.clickMe = this.clickMe.bind(this);
  }
```

First up we call `super` as part of the setup for `constructor`. Now we create our
state variables `count` and `disabled`. `count` will be the number
displayed and `disabled` is for our button. When we click it we
temporarily disable it so that you can't make another request until the
current one is over and the count has been updated.

We then make a request with the `fetch` API (some older browsers might not
support it) that's built into the browser. We grab the response,
turn it into JSON then set the state from the default value of zero
to the actual count from the server. We then bind `this` to `clickMe` so
that it has access to the state inside the function.

```js
  clickMe() {
    this.setState({disabled: true});
    fetch('https://mgattozzi.com/count/', {method: 'PUT'})
      .then((response) => response.json())
      .then((responseJson) => {
        this.setState({count: responseJson.count, disabled: false});
      })
      .catch(console.err);
  }
```

`clickMe` simply does a `PUT` to the server to update the count by one
(we'll see the code for this route soon), and then waits for the
response from the server with the new value. It then changes the count
value and makes the button clickable again. Depending on your
connection speed you might not even see that it becomes disabled!

```js
  render () {
    return(
      <div>
        <div className="panel panel-primary">
          <div className="panel-heading">
            Number of times the button has been clicked
          </div>
          <div className="panel-body text-center">{ this.state.count }</div>
        </div>
        <div>
          <button onClick={this.clickMe}
                  className='btn btn-danger btn-lg center-block'
                  disabled = {this.state.disabled}>
            Click Me!
          </button>
        </div>
      </div>
    );
  }
```

Last but not lease we have our `render` function. This is what renders
the Bootstrap panel and button. Anything inside the curly braces is a
apart of the `Count` component that's added to the output HTML. For
instance `{ this.state.count }` becomes the number value of the count
displayed on screen, the `onClick` feature runs the `clickMe` function
on click and disabled is set to true or false by
`{this.state.disabled}`. It's a simple app for the front end but the
real magic comes from Diesel and Rocket. Let's take a look!

## Building a Diesel Rocket
Alright this part is the more complex part. I had originally set this up
using a connection generated each time upon request. This was
inefficient and after looking at [this issue](https://github.com/SergioBenitez/Rocket/issues/53)
from the Rocket repo I was able to create a version that had a thread pool of database connections
to Postgres using [r2d2](https://github.com/sfackler/r2d2),
[r2d2-diesel](https://github.com/diesel-rs/r2d2-diesel) and the [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs) libraries. Let's
first look at the database library I set up for use within my site.

Here's the code for lib.rs:

```rust
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

pub mod schema;
pub mod models;

use diesel::pg::PgConnection;
use r2d2::{ Pool, Config };
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;

pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let config = Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(config, manager).expect("Failed to create pool.")
}

```

A bit overwhelming but let's go through it. First off we import Diesel
and it's macros for use throughout the database library, which I've called mlib
when imported to the binary running the site, then we import all the
other crates we need like [dotenv](https://github.com/slapresta/rust-dotenv)
to read the .env file for the database url, the r2d2 crate for our pool of
connections, and the r2d2-diesel crate for being able to use r2d2 with Diesel's `PgConnection` type.

After that we publicly declare that we have two modules in the library
that we'll look at soon, they've been labeled `schema` and `models`.
Following that it's all the imports we need to create a `Pool` of database
connections! Let's take a look at that function `create_db_pool`.

```rust
pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let config = Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(config, manager).expect("Failed to create pool.")
}
```

First up we read the .env file and set the environment variables in
there. In this case it's the `DATABASE_URL` variable. We then set that
value as `database_url`. We now set up the pool config (number of
threads etc.) and as you can see I've opted for the default
configuration here. We now set up the manager of all of our connections
and hand it the database url so that it knows where to go to access the
database. We've also told it we want `PgConnections` in our pool. Last
but not least we create our `Pool` of connections and return the value. Neat!

Let's take a look at schema.rs:

```rust
infer_schema!("dotenv:DATABASE_URL");
```

That's it! Diesel does all the hard work figuring out our database
schema for use later. Probably one of my favorite features because this
macro just feels like powerful magic.

Last but not least the models.rs file:

```rust
#[derive(Queryable, Debug)]
pub struct Count {
    pub id: i32,
    pub clicks: i32,
}
```

Since this is the only table in my database my models.rs file isn't
particularly long. We're defining a `struct Count` that contains an id field
and a clicks field. I only have the id to act as a primary key for
lookup and schema purposes. We just want the clicks value usually! We've
also told the `struct` to derive the Diesel `Queryable` trait. This sets
up all the boiler plate code so that we can actually query the database
with it! That's it for the database connection code. Now the actual
binary itself that runs the site!

```rust
#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;
extern crate rocket;
extern crate rocket_contrib;
extern crate diesel;
extern crate mlib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate r2d2;
extern crate r2d2_diesel;

// Server Imports
// Used to Setup DB Pool
use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success, Failure};
use rocket::http::Status;

// Used for Routes
use rocket::Request;
use rocket::response::NamedFile;
use rocket_contrib::JSON;

// Std Imports
use std::path::{Path, PathBuf};

// DB Imports
use diesel::prelude::*;
use diesel::update;
use diesel::pg::PgConnection;
use r2d2::{Pool, PooledConnection, GetTimeout};
use r2d2_diesel::ConnectionManager;
use mlib::models::*;
use mlib::*;

fn main() {
    rocket::ignite().mount("/", routes![count, count_update, public, static_files, index, site]).launch();
}

// DB Items
lazy_static! {
    pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = create_db_pool();
}

pub struct DB(PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
    pub fn conn(&self) -> &PgConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = GetTimeout;
    fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match DB_POOL.get() {
            Ok(conn) => Success(DB(conn)),
            Err(e) => Failure((Status::InternalServerError, e)),
        }
    }
}

// Routes

// I omitted routes here to not clutter everything up

#[get("/count")]
fn count(db: DB) -> JSON<Clicks> {
    use mlib::schema::counts::dsl::*;
    let result = counts.first::<Count>(db.conn())
        .expect("Error loading clicks");

    JSON(Clicks {
        count: result.clicks,
    })
}

#[put("/count")]
fn count_update(db: DB) -> JSON<Clicks> {
    use mlib::schema::counts::dsl::*;
    let query = counts.first::<Count>(db.conn())
        .expect("Error loading clicks");
    let val = query.clicks + 1;

    update(counts.find(1))
        .set(clicks.eq(val))
        .execute(db.conn())
        .unwrap();

    JSON(Clicks {
        count: val,
    })
}

#[derive(Deserialize, Serialize)]
pub struct Clicks {
    pub count: i32,
}

```

Okay, that's a lot. I'm going to break this up into quite a few chunks
for digestion of what's going on. First off let's look at `main`:

```rust
fn main() {
    rocket::ignite().mount("/", routes![count, count_update, public, static_files, index, site]).launch();
}
```

Rocket makes it really easy to just setup routes and run the site. I
don't show all the routes here but most are about just getting the site
via `GET` requests, nothing particularly interesting. What we care about
are the `count` and `count_update` functions. Before that we'll take a look
at how to set up the database for use on those routes.

```rust
// DB Items
lazy_static! {
    pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = create_db_pool();
}

pub struct DB(PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
    pub fn conn(&self) -> &PgConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = GetTimeout;
    fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match DB_POOL.get() {
            Ok(conn) => Success(DB(conn)),
            Err(e) => Failure((Status::InternalServerError, e)),
        }
    }
}
```

Almost all of this I got from the issue I mentioned earlier but I'll
break it down for you:

First off we create a database pool using the `create_db_pool` function
we went over earlier. Why is it wrapped in a `lazy_static!` macro call
though? Well to quote the repo the macro is from:

> <br>
> Using this macro, it is possible to have statics that require code
> to be executed at runtime in order to be initialized.

We want this pool to be available to us for the course of the site's run
time but we can't declare it as being `static` since we
need to first setup our connections. `lazy_static!` allows us to do just
that. Neat huh?

Next we create a `DB struct` that contains one of the connections from
the pool as an inner type. We then create an `impl` for the `DB` type
that has a function `conn` that returns a reference to the `PgConnection` inside
it for use in our requests.

We now setup an `impl` of `FromRequest`, a Rocket trait, for `DB` which is
what we use in this case to get access to the pool in our routes. The
`from_request` function returns our `DB struct` if successful or an
error if it isn't. Cool huh?

Now let's look at the two routes used in the counter:

```rust
#[get("/count")]
fn count(db: DB) -> JSON<Clicks> {
    use mlib::schema::counts::dsl::*;
    let result = counts.first::<Count>(db.conn())
        .expect("Error loading clicks");

    JSON(Clicks {
        count: result.clicks,
    })
}

#[put("/count")]
fn count_update(db: DB) -> JSON<Clicks> {
    use mlib::schema::counts::dsl::*;
    let query = counts.first::<Count>(db.conn())
        .expect("Error loading clicks");
    let val = query.clicks + 1;

    update(counts.find(1))
        .set(clicks.eq(val))
        .execute(db.conn())
        .unwrap();

    JSON(Clicks {
        count: val,
    })
}

#[derive(Deserialize, Serialize)]
pub struct Clicks {
    pub count: i32,
}
```

Alright not too bad in terms of code! You'll notice that both routes
of them have this import in the beginning of the function:

```rust
use mlib::schema::counts::dsl::*;
```

This is what that schema.rs file creates when the macro expands and it
allows us to do things like look things up in the counts table easily!
Okay let's look at `Clicks` first:

```rust
#[derive(Deserialize, Serialize)]
pub struct Clicks {
    pub count: i32,
}
```

This is the JSON that we return to the front end and it's using serde to
serialize and deserialize to and from JSON for us. We use the custom
derive here to avoid doing any of the boiler plate code again. This will
be wrapped inside of Rocket's JSON type to be returned to the front end
to be used as the new count value displayed to you!

Let's look at our `count` function:

```rust
#[get("/count")]
fn count(db: DB) -> JSON<Clicks> {
    use mlib::schema::counts::dsl::*;
    let result = counts.first::<Count>(db.conn())
        .expect("Error loading clicks");

    JSON(Clicks {
        count: result.clicks,
    })
}
```

First up you can declare routes with this syntax:

```rust
#[get("/count")]
```

This tells Rocket that the function below it is used to handle `GET`
requests to the `/count` endpoint. Now if you look at the function
header you'll see that we have an input called `db` that's using our `DB struct`!
This is passed in to the function every time the end point is
requested via a call to `from_request`. Our query is simple, grab the
first field from the `counts` table (it's in the sql file I'll show you soon),
and have it return a `Count` type (the item from models.rs) using a connection retrieved
from having `db` call `conn`, and then unwrap that value using `expect`. In
this case we know the value will be there so this is fine. We then create
a `Clicks struct` that is serialized to JSON and returned to the
frontend! Simple but really cool stuff. Alright now the `PUT` request
route to the same endpoint in our `count_update` function:

```rust
#[put("/count")]
fn count_update(db: DB) -> JSON<Clicks> {
    use mlib::schema::counts::dsl::*;
    let query = counts.first::<Count>(db.conn())
        .expect("Error loading clicks");
    let val = query.clicks + 1;

    update(counts.find(1))
        .set(clicks.eq(val))
        .execute(db.conn())
        .unwrap();

    JSON(Clicks {
        count: val,
    })
}
```

It's similar to before, except now we're saying this route deals with
`PUT` requests. We query the database for the current count. This time though we
take that returned value and increment it by one. We then find the count field in the
database and set it equal to our `val` variable and then return that new
value to the front end to update what is displayed! Pretty cool right?

Now let's look at the migration files. Diesel provides a cool cli tool to
setup the database and run migrations to update it. Here's the
sql that was needed to create the table:

```sql
CREATE TABLE counts (
  id SERIAL PRIMARY KEY,
  clicks SERIAL NOT NULL
);

INSERT INTO counts (clicks) VALUES (0);
```

This was all that was needed to get it running. If I messed up while
writing that all I needed to do was revert the change by running
this sql code via Diesel's cli tool.

```sql
DROP TABLE counts;
```
That's all there is too it! Not so bad now was it?

## Flaws
I do want to take a moment to point out some flaws in this code.
There might be data races if a bunch of people update the count at the
same time in the database (look at the update query to understand why).
The number therefore might not be the most accurate number.
While this is the case and the code could be made more robust
(not using `expect` or `unwrap`) it still accomplishes what I meant to
do which was show Diesel being used in a thread pool with Rocket!

## Conclusion
I'm honestly amazed at how easy it was to get this all working once it
made sense. Rocket is fast and a joy to work with compared to my
experience with Iron and the amount of work on making Diesel such a
high quality library that's easy to work with
has been nothing short of incredible. The work that has been put into
both libraries is amazing. I can't thank the authors of all the crates
I used in this one application enough as they made me productive,
rather than hampering my ability to make what I wanted.
If you want to see the button in action and leave a click it's live
[here](https://mgattozzi.com/counting) and the code for all
of it is at this tag on my repo [here](https://github.com/mgattozzi/mgattozzi/tree/diesel-code-2.0).
Hopefully this gave you a look at what Rocket can do as well as Diesel,
two Rust projects that I really like.

Think my code could use some improvement or you can fix that data race issue I mentioned?
Found a mistake somewhere in the article? Just want to say hi? If so drop by
my [repo](https://github.com/mgattozzi/mgattozzi) and open up a PR or
issue! As always, I hope you learned something new today or got some
inspiration to try something out!
