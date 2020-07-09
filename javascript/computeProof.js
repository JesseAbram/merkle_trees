const ethers = require("ethers")

const hash = (leaf) => {
    return ethers.utils.id(leaf)
}

 const proof =   {
    firstHash: '0x7624778dedc75f8b322b9fa1632a610d40b85e106c7d9bf0e743a9ce291b9c6f',
    nextHash: [
      '0x22be226f1d0a4c116d2025f7870a703bffc378ac8e66b634092acbe31dbfddc9',
      '0xaddd650ef0a5f26ba6b62582caf0ae56b30a3ca207e6095e2a02859c17f7ad76'
    ],
    root: '0x97dfcf5a69636101391dd748545766d180456cd0932dad45b7b37a16498c276c',
    myWord: 'test',
    leafIndex: 1
  }
  
  

 const reduceMerkleBranches = (nextHash, currentHash, i) => {
     //TODO, order hash left vs right
    const output = hash (currentHash + nextHash[i])
    return output

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
    for (i=0; i < proof.nextHash.length; i++) {
         nextLevel =  reduceMerkleBranches(proof.nextHash, nextLevel, i)
    }
   console.log({root})
}

computeProof()