## Merkle Trees 

This is a project I took on as merkle trees are so prevalent in the blockchain space and I always understood them but have never made one before. It also allows me to test out a task in different languages. 

The concept being that each leaf of the tree gets hashed to create nodes and then as the tree collapses upwards by having each node hash with their adjacent node it will end up resolving to a root. 

This root represent all the data of the leaves and to prove that a specific leaf x exists in the tree it would need to have itself and one hash per row of the tree. Representing an exponentially less amount of data per each new row of the tree. 

So far this has been implemented in javascript and rust, with the rust one being done a lot better and with inline comments. 