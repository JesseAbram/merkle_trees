const ethers = require("ethers")

const hash = (leaf) => {
    return ethers.utils.id(leaf)
}

 const proof = {
    adjacentHash: '0x9c22ff5f21f0b81b113e63f7db6da94fedef11b2119b4088b89664fb9a3cb658',
    nextHash: '0x22be226f1d0a4c116d2025f7870a703bffc378ac8e66b634092acbe31dbfddc9',
    root: '0x2d8a8199bc431fc05f1c4d10accfc15d182ea0ff15ec9260b4f7f2e5ef9ac661',
    myWord: 'hi'
  }
  

const computeProof = () => {
   const hashedWord = hash(proof.myWord)
   const nextLevel = hash(hashedWord + proof.adjacentHash)
   const computedRoot = hash(nextLevel + proof.nextHash)
   console.log({computedRoot, hashedWord, nextLevel})
}

computeProof()