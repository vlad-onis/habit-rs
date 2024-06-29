### Installation steps

1. Clone this repo
2. Run the command below 
```bash
# Start the postgres container with default credentials
docker compose -f ./docker/postgres_docker_compose.yml up -d

# After the postgres db started run the below command to create the habits database
./docker/migrations/create_db_if_not_exist.sh
``` 
3. Setup your db by running the migration file
```bash
# TODO:
```
