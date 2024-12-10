#!/usr/bin/env bash

for day in `ls -d day*`; do

(cd $day
  echo "$day"
  cargo run --release
)

done
