#!/bin/bash

for i in $(seq 10 10 100)
do
  ./target/release/rusqlite-multitenancy $i >> results.csv
done
