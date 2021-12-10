# Rails Demo API with Rust

## Description

This is a Demo API that reproduces my **Rails API** in **Rust**.<br>
The **Rails API** repository is [here (Git hub)](https://github.com/takaya787/tut_backend)<br>
The **Rails API's** `open API document` is [here (Swagger hub)](https://app.swaggerhub.com/apis/takaya787/rails-tutorial-api/1.0.0)

## Why I create

I was interested in creating an API in **Rust**, so I tried to create one using `diesel`, a popular **ORM** made by **rust**, and `rocket`, a framework also made by **rust**. <br>
As an **Rails developer**, I have realized how great the `shoulders of the Rails giants` were, and will soon publish an article comparing how to write `Rails` and `Rocket and Diesel`.

## Features

◆ 　 User Functions

- create, delete , login, logout

◆ 　 Micropost Functions

- create, delete , edit

◆ 　 Auth Functions

- login, auto_login(login through the Json web token in header)

◆ 　 Feed Function

- Get microposts of users you follow

## Available URLs

```YAML
paths:
  "/api/login":
    post:
      summary: login
      description: to get the user token by email and password
      tags:
        - Api::Auth
  "/api/auto_login":
    get:
      summary: auto_login
      description: to get the user object through user token
      tags:
        - Api::Auth
  "/api/auto_feed":
    get:
      summary: auto_feed
      description: to get microposts feed of following users
      tags:
        - Api::Auth
  "/api/microposts":
    post:
      summary: create
      tags:
        - Api::Micropost
  "/api/microposts/{id}":
    put:
      summary: update
      tags:
        - Api::Micropost
  "/api/users":
    get:
      summary: index
      tags:
        - Api::User
    post:
      summary: create
      tags:
        - Api::User
```

## Requirements

- rust
- rocket 0.5.0-rc.1
- diesel 1.4.8
- Docker, Docker-compose (開発環境)
- Postgresql(DB)
- dotenv
- chrono
- bcrypt
- serde
- md5
