#!/bin/sh
PACKAGE_NAME=$(head Cargo.toml | awk '/^name/{print $3}' | tr -d '"' | tr -d "'")
./dist/${PACKAGE_NAME}
