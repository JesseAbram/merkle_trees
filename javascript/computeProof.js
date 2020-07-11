const ethers = require("ethers")

const hash = (leaf) => {
    return ethers.utils.id(leaf)
}

const proof = {
    firstHash: '0x686f60ef4d82c89283f9bedf95f9393e4a127390b55cd6f97f937277ad5bc27f',
    nextHash: [
      '0x1ae1455b8158b7e9d7bf73fdc1194481fdec6bd803cb6b3943535d3f2eea780a',
      '0x5348d2e6b276abe051836ae4906f89020ff81b9a6693a697f62264c503e49bab',
      '0x33a85ef0d6c5954bfef9bc4899d9df9d92da9c8891ef7624a862c6e61399f8a2'
    ],
    root: '0xde899aa5cc54e3624d2d5161bb3374b59a7cbcf8bf7acde657c3cdd564ad41ae',
    myWord: 'and',
    leafIndex: 3
  }
  
  
  
  
  
  

 const reduceMerkleBranches = (nextHash, currentHash, leafIndex, i) => {
     //TODO, order hash left vs right
    const newPosition = Math.floor(leafIndex / (i + 2))
    const output = newPosition % 2 === 0 ? hash(currentHash + nextHash[i]) : hash(nextHash[i] + currentHash)
    return {output, newPosition}

}

const computeProof = () => {
   const hashedWord = hash(proof.myWord)
   let nextLevel
   if (proof.leafIndex % 2 === 0) {
       nextLevel = hash(hashedWord + proof.firstHash)
    } else {
        nextLevel = hash(proof.firstHash + hashedWord)
    }
    console.log({nextLevel1:nextLevel})
    let leafIndex = proof.leafIndex
    for (i=0; i < proof.nextHash.length; i++) {
        let {output, newPosition}  =  reduceMerkleBranches(proof.nextHash, nextLevel, leafIndex, i)
        nextLevel = output
        leafIndex = newPosition
    }
   console.log({root: nextLevel})
}

computeProof()