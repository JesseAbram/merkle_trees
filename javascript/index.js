const ethers = require("ethers")

const hash = (leaf) => {
    return ethers.utils.id(leaf)
}

const reduceMerkleBranches = (leaves, hash1) => {
    let output = []
    let i =0
    const adjacentHashIndex = leaves.indexOf(hash1)
    console.log({leaves, hash1})
    while (leaves.length) {
        i++
        let left = leaves.shift()
        let right = (leaves.length === 0) ? left: leaves.shift();
        output.push(hash(left + right)) 
    }
    const nextLevelIndex = Math.floor(adjacentHashIndex / 2) 
    let nextAdjacentHash = nextLevelIndex % 2 === 0 ?  output[nextLevelIndex + 1] : output[nextLevelIndex - 1]
    return {nextLevel: output, returnedNextHash: nextAdjacentHash}

}

const firstHashing = (leaves, leafIndex) => {
    let hashedLeaves = []
    let adjacentHash
    const sisterHashPosition = leafIndex % 2 === 0 ? leafIndex + 1 : leafIndex - 1 
        for(i=0; leaves.length > i; i++) {
            const hashedLeaf =  hash(leaves[i])
            if (i === sisterHashPosition) {
                adjacentHash = hashedLeaf
            }
            hashedLeaves.push(hashedLeaf)
        }
        return {firstLevel: hashedLeaves, adjacentHash}  
} 

const powerOf2Check = (n) => {
   const isPowerOf2 =  n && (n & (n - 1)) === 0
   if (!isPowerOf2) {
       throw new Error("hey wait no stop")
   }
}


const computeRoot = () => {
    let root = []
    let nextHash = []
    const myWord = "and"
    const leaves = ["like", "this", "that", "and", "this", "and", "that", "so", "just", "chill", "till", "the", "next", "episode", "one", "two"]
    const leafIndex = leaves.indexOf(myWord)

    powerOf2Check(leaves.length)
   
    let {firstLevel, adjacentHash} =  firstHashing(leaves, leafIndex)
    const firstHash = adjacentHash
    root.push(...firstLevel)
    while (root.length > 1) {
        const {nextLevel, returnedNextHash} =  reduceMerkleBranches(root, adjacentHash)
        if (returnedNextHash) {
            nextHash.push(returnedNextHash)
            adjacentHash = returnedNextHash
        }
        root.push(...nextLevel)
    }
    console.log("proof", {firstHash, nextHash, root: root[0], myWord, leafIndex})
    console.log({root})

}



computeRoot()
