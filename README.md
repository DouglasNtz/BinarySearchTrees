# BinarySearchTrees

Árvore de busca binária de elementos (key, value) baseados no valor de key e na sua ordem de inserção.

Sempre que uma chave key é inserida, a regra é:
- se key < node então vai para left child
- se key >= node então vai para right child

Chaves repetidas são permitidas, mas no metodo get e get_mut é retornado value da primeira delas em ordem de inserção

O mesmo vale para deletion. A primeira chave com valor key encontrada será deletada. Ou seja, a primeira inserida será 

a primeura a ser deletada.

O metodo para reestruturação da árvore no caso de deltetion de node com dois filhos é o transplante, o qual

coloca no lugar o sucessor do node deletado.
