from sets import Set
import numpy as np 

class UndirectedGraph:

    def __init__(self):
        print("\n ---------------------- Undirected Graph Library, 2020 ---------------------- \n")
        self.vertices = Set()
        self.edges = Set()

    #Duplicates are handled by the set library
    def addNode(self, itemKey, itemVal, edgeNodes):
        if (self.findNode(itemKey) != -1):
            print("A node was not added. Key '" + itemKey + "' is already in the graph. Consider update node value instead. \n")
        else:    
            self.vertices.add((itemKey, itemVal))
            for node in edgeNodes:
                #To prevent duplicates
                first = min(itemKey, node)
                second = max(itemKey, node)
                self.edges.add((first, second))
    
    #Draw based on key
    def drawEdge(self, leftNode, rightNode):
        first = min(leftNode, rightNode)
        second = max(leftNode, rightNode)
        self.edges.add((first, second))

    #Find node based on key
    def findNode(self, nodeKey):
        for graphNode in self.vertices:
            if graphNode[0] == nodeKey:
                return graphNode
            else:
                return -1
        return -1

    def updateNodeValue(self, nodeKey, nodeVal):
        node = self.findNode(nodeKey)
        if node == -1:
            return -1
        else:
            self.vertices.remove(node)
            self.vertices.add((nodeKey, nodeVal))
            #print("Updated '" + nodeKey + "' to value: " + nodeVal + ". \n")

    def areConnected(self, leftNodeKey, rightNodeKey):
        first = min(leftNodeKey, rightNodeKey)
        second = max(leftNodeKey, rightNodeKey)
        if (first, second) in self.edges:
            #print("Found connected edges: '" + str(first) + "' and '" + str(second) + "'.\n")
            return True
        else:
            #print("Edges '" + str(leftNodeKey) + "' and '" + str(rightNodeKey) + "' are not connected.\n")
            return False

    def addItem(self, itemVal, edgeNodes):
        itemKey = hash(itemVal)
        self.addNode(itemKey, itemVal, edgeNodes)

    def itemKey(self, itemVal):
        return hash(itemVal)


    #Find the length of a path between two nodes, or -1 if one does not exist.
    def pathLength(self, vertices, nodeAKey, nodeBKey, i = 0):
        edges = self.edges
        aConnected = Set()
        for vertex in vertices:
            if (self.areConnected(vertex[0], nodeAKey)):
                i += 1
                if(vertex[0] == nodeBKey): 
                    return i
                aConnected.add(vertex)
        for vertex in aConnected:
            i = self.pathLength(vertices.difference(aConnected), vertex[0], nodeBKey, i)
        if (len(aConnected) == 0):
            i = -1
        return i


g = UndirectedGraph()
edges = Set()
g.addNode("B", "text B", edges)
g.addNode("A", "text A", edges)
g.addNode("C", "text C", edges)

g.addItem("Josie", edges)
g.addNode("C", "yabababa", edges)
g.updateNodeValue("C", "Scrappy doo")
g.addNode("D", "text D", "A")
g.drawEdge("A", "C")
g.areConnected("A", "C")
g.areConnected("C", "B")
key = g.itemKey("Josie")
g.drawEdge(key, "A")
g.areConnected(key, "A")
g.areConnected(key, "B")
print(g.pathLength(g.vertices, "A", "C"))
print(g.pathLength(g.vertices, "D", "C"))
print(g.pathLength(g.vertices, "D", "B"))
print(g.vertices)
print(g.edges)