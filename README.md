#  Rails Demo API with Rust

## Description
これは、自分が作成した**Rails API**を**Rust**で再現したDemo APIです。<br>
**Rails API**のレポジトリーは[こちら](https://github.com/takaya787/tut_backend)<br>
**Rails API**の仕様書は[こちら(Swagger hub)](https://app.swaggerhub.com/apis/takaya787/rails-tutorial-api/1.0.0)

## Implemented URLs

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
      description: to get feed micropost feeds
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
* rust
* rocket 0.5.0-rc.1 
* diesel 1.4.8
* Docker, Docker-compose (開発環境)
* Postgresql(DB)

## 機能一覧
◆　ユーザー機能 
* 新規登録、ログイン、ログアウト

◆　マイクロポスト機能
* マイクロポスト作成、編集、消去

◆　フィード機能
* フォローしたユーザーのマイクロポストを表示
