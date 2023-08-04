#--------------------------------- data ---------------------------------
@problems = []  

problemsTxt = File.open("21_Complete_Data/problems.txt").read
i = 1
for prob in problemsTxt.split("\n")
    @problems << prob
    i = i + 1
end

def genAdjacencyMatrix()
    reductions = File.open("21_Complete_Data/reductions.csv").read
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

@adjacencyMatrix = genAdjacencyMatrix()

#--------------------------------- helper functions ---------------------------------


def displayProblems()
    p 'Type "H" or "help" to retrieve this list at any point.'
    
    i = 1
    for prob in @problems
        p "(" + i.to_s + ") " + prob
        i = i + 1
    end
    p "--------------"
end

def displayReductions()
    #21 problems
    p "Displaying reductions --------------"
    i = 1
    for row in 0..20
        for col in 0..20
            if @adjacencyMatrix[row][col] != 0
                p i.to_s + ". " + @problems[row] + " reduces to " + @problems[col]
                i = i + 1
            end
        end
    end
    p "--------------"
end

def processInput()
    text = gets
    if text == "H\n" or text == "help\n"
        displayReductions()
        text = processInput()
    end
    return text.to_i
end


def writeReductionsCSV()
    text = ""
    for row in @adjacencyMatrix
        for col in row
            text += col.to_s + ","
        end
        text = text.chop()
        text = text + "\n"
    end
    text = text.chop()
    File.write("21_Complete_Data/reductions.csv", text)
end

def updateEntry(from, to, val) 
    @adjacencyMatrix[from-1][to - 1] = val #0, 1, 2, or 3 depending on input
    writeReductionsCSV()
end

def addReduction() 
    p "What are you reducing from: "
    from = processInput()
    p "What are you reducing to?"
    to = processInput()

    if @adjacencyMatrix[from - 1, to - 1] != 0
        p "That reduction is already included!"
    else 
        p "Is this reduction (1) original, (2) Due to Karp, or (3) from somewhere else in the literature?"
        val = processInput()
        updateEntry(from, to, val)
    end

    writeReductionsCSV()
end

def updateReduction() 
    p "Which reduction do you want to update?"
    reductionNumber = processInput()

    idx = 1
    ctr = 1
    stopHere = nil
    #Confirmed experimentally: Row contains the data for the thing that we are reducing from
    for row in 0..20
        for col in 0..20
            if @adjacencyMatrix[row][col] != 0
                if idx == reductionNumber
                    stopHere = ctr
                end
                #Update after to get the same number as given in display
                idx = idx + 1
            end
            ctr = ctr + 1
        end
    end

    #Jo: I feel good about to_idx now. The nice "to" number is that plus one.
    to_idx = ((stopHere - 1) % 21) #From is row number
    to = to_idx + 1
    from_idx = (stopHere - to)/21 #To is column number
    from = from_idx + 1

    p "Do you want to update to (0) remove reduction or (2) update its status?"
    val = processInput()
    if val == 0
        updateEntry(from, to, 0)
    else
        p "Ok. Do you want to save the reduction as (1) original, (2) due to Karp, or (3) From somewhere else in the literature?"
        val = processInput()
        updateEntry(from, to, val)
    end
end

#--------------------------------- Run-time logic ---------------------------------
displayProblems()

val = 1
while val != 0
    p "Do you want to (1) Add reduction, (2) Update existing, (3) Display reductions, or (0) close?"
    val = processInput()
    if val == 1 
        addReduction()
    elsif val == 2 || val == 3
        displayReductions()
        if val == 2 
            updateReduction()
        end
    end
end