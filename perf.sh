#!/bin/bash
#sudo /home/andrew/.cargo/bin/flamegraph --palette js -o runjs-perf-slow.svg -- $(pwd)/target/perf/runjs bench.js
#sudo /home/andrew/.cargo/bin/flamegraph --palette js -o runjs-perf-fast.svg -- $(pwd)/target/perf/runjs bench.js
sudo /home/andrew/.cargo/bin/flamegraph --palette js -o deno_encode_into.svg -- ./deno run -A --unstable --v8-flags="--perf-basic-prof" bench.js 5 1400000 deno_encode_into
sudo /home/andrew/.cargo/bin/flamegraph --palette js -o runjs_encode_into.svg -- ./target/release/runjs bench.js 5 2500000 runjs_encode_into
sudo /home/andrew/.cargo/bin/flamegraph --palette js -o runjs_encode_into_fast.svg -- ./target/release/runjs bench.js 5 5000000 runjs_encode_into_fast
