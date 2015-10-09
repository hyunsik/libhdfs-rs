#!/bin/bash

for each_file in `find proto/ -type f`; do
  protoc -Iproto/ --rust_out src/proto $each_file
done
