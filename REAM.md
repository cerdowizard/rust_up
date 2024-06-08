## create a new instance of env

run this to start the docker daemon
docker-compose up -d

### Assuming you have already installed the SQLx-CLI binary, you can use the following command in the project’s root directory to generate reversible migrations with corresponding ‘up‘ and ‘down‘ 
```bash 
    scripts: 
    sqlx migrate add -r init
    sqlx migrate run
    cargo watch -q -c -w src/ -x run
