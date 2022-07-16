#!/bin/sh 

cargo release --workspace --exclude book --exclude examples $@
