#!/bin/sh

echo "Running add reduction scripts"
ruby addReduction.rb
cd scc-rust/src
cargo run
cd ../../21_Complete_Data
dot -Tpdf graph.dot -o graph.pdf
