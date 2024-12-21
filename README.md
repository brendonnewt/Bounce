# Bounce

Thanks for stopping by! This project is a Trampoline & Tumbling Training Logger API designed for athletes to be able to log their training sessions. This API also allows coaches and athletes to link themselves to clubs so that coaches can view their athletes training logs!

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
