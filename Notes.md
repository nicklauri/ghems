
local postgres connection:
    - using docker.
    - connection cmdline:
        psql -h localhost -p 49153 -U postgres
        password: postgrespw


\l      : list dbs
\i file : execute a file
\dt     : list tables
\dt+    : list tables

\!      : run shell command

grant table privileges to role:
    grant all privileges on all tables in schema public to ghem_app;


apply: #[axum_macros::debug_handler] to handler to see better error message

curl localhost/api/user/login -v -X POST -H 'content-type: application/json' --data '{"username": "admin", "password": "admin"}' ; echo

curl localhost:8080/api/user -v -X POST -H 'content-type: application/json' -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzM4NCJ9.eyJ1c2VyX2lkIjoiM2Q3MjFiY2MtNGVjMi00ZWZiLTk2ZWMtMmVlMmY2ZmM2MzNjIiwic3ViIjoiZ2hlbXNfYWRtaW5AbWFpbGluYXRvci5jb20iLCJleHAiOjE2NjA0MDEyOTR9.iDJqwnvdRR333GSms9ru7CGnfPMYtxnwSAsJreAub1BBMZ3KLAZk67L0KplDrg2s'

curl localhost/api/user -v -X POST -H 'content-type: application/json' -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzM4NCJ9.eyJ1c2VyX2lkIjoiMDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtZmYwMDAwMDAwMDAyIiwic3ViIjoia250YUBtYWlsaW5hdG9yLmNvbSIsImV4cCI6MTY2MjM5Mzg4OCwicm9sZXMiOlsiQWRtaW4iLCJNZW1iZXIiXX0._3W44TkLXBSWqXGGumG3VTxgsgx4l51W2OnBg3cEISZ5Zd9Ftl515AnjBTuXHuPI'

curl localhost:8080/api/user/member -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzM4NCJ9.eyJ1c2VyX2lkIjoiMDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtZmYwMDAwMDAwMDAyIiwic3ViIjoia250YUBtYWlsaW5hdG9yLmNvbSIsImV4cCI6MTY2MjM5NDM4Nywicm9sZXMiOlsiYWRtaW4iLCJtZW1iZXIiXX0.IZyZqUaf81RkgXnCMSxJOXXqKe9fmpa77atTGX1iJEdJUoCP9q9mSwl0tzIpzzVi' -H 'content-type: application/json' -v

curl localhost:8080/api/user/admin -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzM4NCJ9.eyJ1c2VyX2lkIjoiMDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtZmYwMDAwMDAwMDAyIiwic3ViIjoia250YUBtYWlsaW5hdG9yLmNvbSIsImV4cCI6MTY2MjM5NDM4Nywicm9sZXMiOlsiYWRtaW4iLCJtZW1iZXIiXX0.IZyZqUaf81RkgXnCMSxJOXXqKe9fmpa77atTGX1iJEdJUoCP9q9mSwl0tzIpzzVi' -H 'content-type: application/json' -v

button: box-shadow: rgba(0, 0, 0, 0.12) 0px 1px 3px, rgba(0, 0, 0, 0.24) 0px 1px 2px;
card: box-shadow: rgba(0, 0, 0, 0.1) 0px 10px 50px;

cool:
    box-shadow: rgba(240, 46, 170, 0.4) -5px 5px, rgba(240, 46, 170, 0.3) -10px 10px, rgba(240, 46, 170, 0.2) -15px 15px, rgba(240, 46, 170, 0.1) -20px 20px, rgba(240, 46, 170, 0.05) -25px 25px;

    box-shadow: rgba(240, 46, 170, 0.4) 0px 5px, rgba(240, 46, 170, 0.3) 0px 10px, rgba(240, 46, 170, 0.2) 0px 15px, rgba(240, 46, 170, 0.1) 0px 20px, rgba(240, 46, 170, 0.05) 0px 25px;

    box-shadow: rgba(240, 46, 170, 0.4) 5px 5px, rgba(240, 46, 170, 0.3) 10px 10px, rgba(240, 46, 170, 0.2) 15px 15px, rgba(240, 46, 170, 0.1) 20px 20px, rgba(240, 46, 170, 0.05) 25px 25px;


https://github.dev/neoeinstein/aliri/blob/main/aliri_axum/examples/auth0_server.rs


#anyhow = "1.0.61"
# argon2 = { version = "0.4.1", features = ["rayon"] }
# async-trait = "0.1.57"
# axum = { version = "0.5.15" }
# axum-macros = "0.2.3"
# base64 = "0.13.0"
# clap = { version = "3.2.17", features = ["env", "derive"] }
# dashmap = { version = "5.3.4", features = ["rayon", "serde"] }
# dotenvy = { version = "0.15.1", features = ["clap"] }
# jsonwebtoken = "8.1.1"
# mimalloc = "0.1.29"
# once_cell = "1.13.0"
# rand = "0.8.5"
# serde = { version = "1.0.143", features = ["derive"] }
# serde_json = "1.0.83"
# sha2 = "0.10.2"
# sqlx = { version = "0.6.1", features = ["postgres", "runtime-tokio-rustls", "uuid", "time"] }
# thiserror = "1.0.32"
# time = { version = "0.3.13", features = ["serde"] }
# tokio = { version = "1.20.1", features = ["full"] }
# tower-http = { version = "0.3.4", features = ["full"] }
# tracing = "0.1.36"
# tracing-subscriber = { version = "0.3.15", features = ["std", "env-filter"] }
# uuid = { version = "1.1.2", features = ["serde"] }
# validator = { version = "0.16.0", features = ["derive"] }