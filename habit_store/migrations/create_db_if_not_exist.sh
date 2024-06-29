
# Verifies if the db exists and if not it creates it
psql -h 0.0.0.0 -p 25432 -U postgres -tc "SELECT 1 FROM pg_database WHERE datname = 'my_db'" |
  grep -q 1 ||
  psql -h 0.0.0.0 -p 25432 -U postgres -c "CREATE DATABASE habits_db"
