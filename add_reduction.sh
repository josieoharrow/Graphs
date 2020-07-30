#!/bin/sh

echo "Running add reduction scripts"
ruby addReduction.rb
cd scc-rust/src
cargo run
cd ../../21_Complete_Data
dot -Tps graph.dot -o graph.ps
ps2pdf graph.ps
