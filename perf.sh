#!/bin/bash
#sudo /home/andrew/.cargo/bin/flamegraph --palette js -o runjs-perf-slow.svg -- $(pwd)/target/perf/runjs bench.js
sudo /home/andrew/.cargo/bin/flamegraph --palette js -o runjs-perf-fast.svg -- $(pwd)/target/perf/runjs bench.js
