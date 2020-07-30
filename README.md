# Graphs
The Graphs Repo is currently serving as a combination of mathematical graph code, and research code.

## Non-Project Code
There is currently only undirected_graph.py which is part of non-project code. undirected_graph.py provides an undirected graph class in Python.

## Project Code
The project code is divided into two main sections, maintaining the reductions data, and checking reductions and generating the reduction graph.

### Reductions Data
Data containing problems to reduce is in problems.txt. Data containing the reductions is in reductions.csv (rows are reduced to columns). Because csv is easier to parse but harder to edit and upkeep, I wrote a helper script addReduction.rb to maintain the csv. I could use another one (removeReduction).

### Working with the graph
The main.rs file is responsible for both checking for a cycle containing all nodes, and updating the graph.dot file to show any new reductions (should that happen immediateley after the addReduction step?) After that, the user is responsible for creating the .ps file using graphvis, and then using that however they see fit (for me, another coversion to pdf works fine).

## Add-Ons
Some useful things to add:
- Simple cycle checker (DHC)
- Automation for generating the .ps file *after* addReduction is run.
- removeReduction
- Better labels for the graph nodes
- Visualization on the graph for which edges we added, and which were from Karp
- More project organization, this project is somewhat of an engineering mess currently and could use some better organization if anyone is going to spend time on it.