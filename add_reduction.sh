#!/bin/sh

echo "Running add reduction scripts"
ruby addReduction.rb
cd scc-rust/src
cargo run
cd ../../21_Complete_Data
dot -Tps graph.dot -o graph.ps
gs -o graph.pdf -sDEVICE=pdfwrite -g9775x9207px -dPDFFitPage graph.ps