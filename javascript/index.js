const ethers = require("ethers")

const hash = (leaf) => {
    return ethers.utils.id(leaf)
}

const reduceMerkleBranches = (leaves, adjacentHash) => {
    let output = []
    let nextHash
    let i =0
    while (leaves.length) {
        i++
        let left = leaves.shift()
        let right = (leaves.length === 0) ? left: leaves.shift();
        output.push(hash(left + right)) 
        if (left !== adjacentHash || right !== adjacentHash) {
            nextHash = i/2
        }
    }
    return {nextLevel: output, returnedNextHash: output[nextHash]}

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
    let nextHash
    const myWord = "test"
    const leaves = ["hi", "test", "thing", "this"]
    const leafIndex = leaves.indexOf(myWord)

    powerOf2Check(leaves.length)
   
    const {firstLevel, adjacentHash} =  firstHashing(leaves, leafIndex)
    root.push(...firstLevel)
    while (root.length > 1) {
        const {nextLevel, returnedNextHash} =  reduceMerkleBranches(root, adjacentHash)
        if (returnedNextHash) {
            nextHash = returnedNextHash
        }
        root.push(...nextLevel)
    }
    console.log("proof", {adjacentHash, nextHash, root: root[0], myWord, leafIndex})
    console.log({root})

}



computeRoot()
