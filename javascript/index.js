const ethers = require("ethers")

const hash = async (leaf) => {
    return ethers.utils.id(leaf)
}

const reduceMerkleBranches = async (leaves) => {
    let output = []

    while (leaves.length) {
        let left = leaves.shift()
        let right = (leaves.length === 0) ? left: leaves.shift();
        output.push(await hash(left + right)) 
    }
    return output

}

const firstHashing = async (leaves) => {
    let hashedLeaves = []
        for(i=0; leaves.length > i; i++) {
            const news = await hash(leaves[i])
            hashedLeaves.push(news)
        }
        return hashedLeaves  
} 

const computeRoot = async () => {
    let root = []
    const leaves = ["hi", "test", "thing", "this", "whoa", "true"]
    if(leaves.length % 2 !== 0) {
        throw new Error ("hey stop")
    }
    const firstLevel = await firstHashing(leaves, true)
    root.push(...firstLevel)

    while (root.length > 1) {
        const nextLevel = await reduceMerkleBranches(root)
        root.push(...nextLevel)
    }

    console.log({root})

}


computeRoot()
