#!/bin/bash

test_databases_file=/tmp/test_dbs.txt
psql -d postgres -c "COPY (SELECT datname FROM pg_database WHERE datname LIKE 'test-shrtnr-db-%' AND datistemplate=false) TO '$test_databases_file'"

while read dbname
do
  echo "dropping DB $dbname..."
  dropdb "$dbname"
done < $test_databases_file

echo "removing $test_databases_file file"
rm $test_databases_file
