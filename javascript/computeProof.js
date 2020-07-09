const ethers = require("ethers")

const hash = (leaf) => {
    return ethers.utils.id(leaf)
}

 const proof = {
    adjacentHash: '0x7624778dedc75f8b322b9fa1632a610d40b85e106c7d9bf0e743a9ce291b9c6f',
    nextHash: '0x22be226f1d0a4c116d2025f7870a703bffc378ac8e66b634092acbe31dbfddc9',
    root: '0x2d8a8199bc431fc05f1c4d10accfc15d182ea0ff15ec9260b4f7f2e5ef9ac661',
    myWord: 'test',
    leafIndex: 1
  }
  

const computeProof = () => {
   const hashedWord = hash(proof.myWord)
   if (proof.leafIndex % 2 === 0) {
       nextLevel = hash(hashedWord + proof.adjacentHash)
    } else {
        nextLevel = hash(proof.adjacentHash + hashedWord)
    }

   const computedRoot = hash(nextLevel + proof.nextHash)
   console.log({computedRoot, hashedWord, nextLevel})
}

computeProof()