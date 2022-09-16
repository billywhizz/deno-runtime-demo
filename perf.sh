#!/bin/bash
sudo /home/andrew/.cargo/bin/flamegraph --palette js -o runjs-perf.svg -- $(pwd)/target/release/runjs bench.js
