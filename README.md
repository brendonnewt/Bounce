# Bounce

Thanks for stopping by! This project is a Trampoline & Tumbling Training Logger API designed for athletes to be able to log their training sessions. This API also allows coaches and athletes to link themselves to clubs so that coaches can view their athletes training logs!

## Table of Contents

- [Setup Instructions](#setup-instructions)
  - [Environment Variables](#environment-variables)
  - [Database Setup](#database-setup)
  - [Running the App](#running-the-app)
- [Features](#api-features)
  - [User Features](#users-can)
  - [Coach Features](#coaches-can)

## Setup Instructions

### Environment Variables

You'll need to create a .env file in the api directory with the following contents

1. ADDRESS - The address for the Actix Web server to run on
2. PORT - The port the server should expose
3. DATABASE_URL - The URL for your SQLite database

### Database Setup

Once the environment variables have been configured, migrate the database with

```bash
    sea-orm-cli migrate up
```

This will create the necessary tables in the database specified in $DATABASE_URL

### Running the App

After setting up the env file and migrating the database, you can start the actix web server by
running the following command from the api directory

```bash
    cargo run
```

This will start the server on the port and host specified in the .env file

## API Features

### Users can...

1. Create a coach or athlete account
2. Join a club
3. Leave a club
4. Update profile info
5. Reset their password
6. Retrieve their basic profile info

### Coaches can...

1. Own a club
2. Delete a club
3. Transfer ownership of a club to another coach
