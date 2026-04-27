#!/usr/bin/env bash

i=0
while [ $i -le 22221 ]; do
echo  "{\"jobs_processed\": $i}" > "job-$i.json"
((i++))
done

echo "Created $i job files"
