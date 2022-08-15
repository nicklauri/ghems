
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


apply: #[axum_macros::debug_handler] to handler to see better error message

curl localhost/api/user/login -v -X POST -H 'content-type: application/json' --data '{"username": "admin", "password": "admin"}' ; echo

curl localhost/api/user -v -X POST -H 'content-type: application/json' -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzM4NCJ9.eyJ1c2VyX2lkIjoiM2Q3MjFiY2MtNGVjMi00ZWZiLTk2ZWMtMmVlMmY2ZmM2MzNjIiwic3ViIjoiZ2hlbXNfYWRtaW5AbWFpbGluYXRvci5jb20iLCJleHAiOjE2NjA0MDEyOTR9.iDJqwnvdRR333GSms9ru7CGnfPMYtxnwSAsJreAub1BBMZ3KLAZk67L0KplDrg2s'


button: box-shadow: rgba(0, 0, 0, 0.12) 0px 1px 3px, rgba(0, 0, 0, 0.24) 0px 1px 2px;
card: box-shadow: rgba(0, 0, 0, 0.1) 0px 10px 50px;

cool:
    box-shadow: rgba(240, 46, 170, 0.4) -5px 5px, rgba(240, 46, 170, 0.3) -10px 10px, rgba(240, 46, 170, 0.2) -15px 15px, rgba(240, 46, 170, 0.1) -20px 20px, rgba(240, 46, 170, 0.05) -25px 25px;

    box-shadow: rgba(240, 46, 170, 0.4) 0px 5px, rgba(240, 46, 170, 0.3) 0px 10px, rgba(240, 46, 170, 0.2) 0px 15px, rgba(240, 46, 170, 0.1) 0px 20px, rgba(240, 46, 170, 0.05) 0px 25px;

    box-shadow: rgba(240, 46, 170, 0.4) 5px 5px, rgba(240, 46, 170, 0.3) 10px 10px, rgba(240, 46, 170, 0.2) 15px 15px, rgba(240, 46, 170, 0.1) 20px 20px, rgba(240, 46, 170, 0.05) 25px 25px;


https://github.dev/neoeinstein/aliri/blob/main/aliri_axum/examples/auth0_server.rs