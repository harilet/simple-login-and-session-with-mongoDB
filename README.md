# simple login and session with mongoDB in rust

src > db > mod.rs Change mongodb://localhost:8081 to your MongoDB Database URL

## index page
The index page shows Hello, world! if not logged-in
And shows Hello, <username>!

## /users?username=<username> url
Shows user's username and ID

## /users url post request
Post request to /users will add user to database

Body:
```
{
    username:<username>,
    password:<password>
}
```

## /login url post request
Post request to /login login user

Body:
```
{
    username:<username>,
    password:<password>
}
```

## /logout url
Log user out of server
