@problems = []

def displayReductions()
    p 'Type "H" or "help" to retrieve this list at any point.'
    
    problems = File.open("problems.txt")
    problemsTxt = problems.read
    i = 1
    for prob in problemsTxt.split("\n")
        p "(" + i.to_s + ") " + prob
        @problems << prob
        i = i + 1
    end
    p "---------"
end

def processInput()
    text = gets
    if text == "H\n" or text == "help\n"
        displayReductions()
        text = processInput()
    end
    return text.to_i
end

displayReductions()

p "What are you reducing from: "
from = processInput()
p "What are you reducing to?"
to = processInput()

def genAdjacencyMatrix()
    reductions = File.open("reductions.csv").read
    reductions = reductions.split("\n")
    m = Array.new(21) { Array.new(21) }
    r = 0
    for row in reductions
        columns = row.split(",")
        c = 0
        for entry in columns
            m[r][c] = entry.to_i
            c = c + 1
        end
        r = r + 1
    end
    return m
end

def writeReductionsCSV(matrix)
    text = ""
    for row in matrix
        for col in row
            text += col.to_s + ","
        end
        text = text.chop()
        text = text + "\n"
    end
    text = text.chop()
    File.write("reductions.csv", text)
end

m = genAdjacencyMatrix()

if m[from - 1, to - 1] == 1
    p "That reduction is already included!"
else 
    m[from-1][to - 1] = 1
    writeReductionsCSV(m)
end

m = genAdjacencyMatrix()
