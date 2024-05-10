use std::fmt::Debug;

#[derive(Debug)]
struct Element<T: PartialOrd + Debug, E: Debug> {
    value: T,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    satelite: E
}

#[derive(Debug)]
pub struct BinarySearchTree<T: PartialOrd + Debug, E: Debug> {
    root: Option<usize>,
    array: Vec<Element<T,E>>
}

impl<T: PartialOrd + Debug, E: Debug> BinarySearchTree<T,E> {
    pub fn new() -> Self {

        Self { root: None, array: Vec::new() }
    }

    pub fn insert(self: &mut Self, value: T, satelite: E) {

        let vlen = self.array.len();

        match self.root {
            None => {
                let element = Element {value, parent: None, left: None, right: None, satelite};
                self.root = Some(0);
                self.array.push(element);
            }
            Some(mut i) => {
                loop {
                    if value < self.array[i].value {
                        match self.array[i].left {
                            Some(j) => {
                                i = j;
                                continue;
                            }
                            None => {
                                let element = Element {value, parent: Some(i), left: None, right: None, satelite};
                                self.array.push(element);
                                self.array[i].left = Some(vlen);
                                break;
                            }
                        }
                    } else {
                        match self.array[i].right {
                            Some(j) => {
                                i = j;
                                continue;
                            }
                            None => {
                                let element = Element {value, parent: Some(i), left: None, right: None, satelite};
                                self.array.push(element);
                                self.array[i].right = Some(vlen);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn minimum(self: &Self) -> Option<&T> {
        match self.root {
            None => {
                println!("Árvore não contém elementos.");
                None
            }
            Some(mut i) => {
                loop {
                    match self.array[i].left {
                        Some(j) => {
                            i = j;
                            continue;
                        }
                        None => {
                            break Some(&self.array[i].value)
                        }
                    }
                }
            }
        }
    }

    pub fn maximum(self: &Self) -> Option<&T> {
        match self.root {
            None => {
                println!("Árvore não contém elementos.");
                None
            }
            Some(mut i) => {
                loop {
                    match self.array[i].right {
                        Some(j) => {
                            i = j;
                            continue;
                        }
                        None => {
                            break Some(&self.array[i].value)
                        }
                    }
                }
            }
        }
    }

    pub fn get(self: &Self, value: &T) -> Option<&E> {  // retorna a primeira chave igual a value encontrada

        match self.root {
            None => {
                println!("Valor procurado não existe na árvore.");
                None
            }
            Some(mut i) => {
                loop {
                    if value < &self.array[i].value {
                        match self.array[i].left {
                            Some(j) => {
                                i = j;
                                continue;
                            }
                            None => {
                                println!("Valor procurado não existe na árvore.");
                                break None
                            }
                        }
                    } else if value > &self.array[i].value{
                        match self.array[i].right {
                            Some(j) => {
                                i = j;
                                continue;
                            }
                            None => {
                                println!("Valor procurado não existe na árvore.");
                                break None
                            }
                        }
                    } else {
                        break Some(&self.array[i].satelite)
                    }
                }
            }
        }
    }

    pub fn get_mut(self: &mut Self, value: &T) -> Option<&mut E> {  // retorna a primeira chave igual a value encontrada

        match self.root {
            None => {
                println!("Valor procurado não existe na árvore.");
                None
            }
            Some(mut i) => {
                loop {
                    if value < &self.array[i].value {
                        match self.array[i].left {
                            Some(j) => {
                                i = j;
                                continue;
                            }
                            None => {
                                println!("Valor procurado não existe na árvore.");
                                break None
                            }
                        }
                    } else if value > &self.array[i].value{
                        match self.array[i].right {
                            Some(j) => {
                                i = j;
                                continue;
                            }
                            None => {
                                println!("Valor procurado não existe na árvore.");
                                break None
                            }
                        }
                    } else {
                        break Some(&mut self.array[i].satelite)
                    }
                }
            }
        }
    }

    fn get_index(self: &Self, value: &T) -> Option<usize> {  // retorna a primeira chave igual a value encontrada

        match self.root {
            None => {
                println!("Valor procurado não existe na árvore.");
                None
            }
            Some(mut i) => {
                loop {
                    if value < &self.array[i].value {
                        match self.array[i].left {
                            Some(j) => {
                                i = j;
                                continue;
                            }
                            None => {
                                println!("Valor procurado não existe na árvore.");
                                break None
                            }
                        }
                    } else if value > &self.array[i].value{
                        match self.array[i].right {
                            Some(j) => {
                                i = j;
                                continue;
                            }
                            None => {
                                println!("Valor procurado não existe na árvore.");
                                break None
                            }
                        }
                    } else {
                        break Some(i)
                    }
                }
            }
        }
    }


    pub fn inorder(self: &Self) -> Vec<(&T,&E)> {  // estável de acordo com a ordem de insert (ou seja, por altura na árvore)

        let mut w = vec![];

        match self.root {
            Some(i) => {
                self.inorder_aux(i)
            }
            None => w
        }
    }

    fn inorder_aux(self: &Self, index: usize) -> Vec<(&T,&E)> {

        let mut w = vec![];

        match self.array[index].left {
            Some(i) => {
                let mut wl = self.inorder_aux(i);
                w.append(&mut wl);
            }
            None => {}
        }

        w.push((&self.array[index].value, &self.array[index].satelite));

        match self.array[index].right {
            Some(i) => {
                let mut wr = self.inorder_aux(i);
                w.append(&mut wr);
            }
            None => {}
        }
        w
    }

    pub fn len(self: &Self) -> usize {

        self.array.len()
    }
    pub fn deletion(self: &mut Self, value: &T) -> Option<E> {  // deleta o primeiro encontrado com chave value encontrada

        let opt_index = self.get_index(value);

        let index = match opt_index {
            Some(i) => i,
            None => return None
        };


        let last = self.array.len() - 1;

        if index != last {

            self.array.swap(index, last);  // troca de posição com o último no array

            match self.array[index].left {
                Some(i) => {
                    self.array[i].parent = Some(index)
                },
                None => {}
            }
            match self.array[index].right {
                Some(i) => {
                    self.array[i].parent = Some(index)
                },
                None => {}
            }
            match self.array[index].parent {
                Some(i) => {
                    if self.array[index].value < self.array[i].value {
                        self.array[i].left = Some(index);
                    } else {
                        self.array[i].right = Some(index);
                    }
                },
                None => self.root = Some(index)
            }

            match self.array[last].left {
                Some(i) => {
                    self.array[i].parent = Some(last)
                },
                None => {}
            }
            match self.array[last].right {
                Some(i) => {
                    self.array[i].parent = Some(last)
                },
                None => {}
            }
            match self.array[last].parent {
                Some(i) => {
                    if self.array[last].value < self.array[i].value {
                        self.array[i].left = Some(last);
                    } else {
                        self.array[i].right = Some(last);
                    }
                },
                None => self.root = Some(last)
            }
        }

        // corrigimos os pais e filhos. Agora quando for fazer pop(), não estraga os índices do array
        // Basta reconfigurar quem é pai e filho de quem.

        match (self.array[last].left, self.array[last].right) {
            (None, None) => {
                match self.array[last].parent {
                    Some(i) => {
                        if self.array[last].value < self.array[i].value {
                            self.array[i].left = None;
                        } else {
                            self.array[i].right = None;
                        }
                    },
                    None => self.root = None
                }
            },
            (None, Some(j)) => {
                match self.array[last].parent {
                    Some(i) => {
                        if self.array[last].value < self.array[i].value {
                            self.array[i].left = Some(j);
                        } else {
                            self.array[i].right = Some(j);
                        }
                        self.array[j].parent = Some(i);
                    },
                    None => {
                        self.root = Some(j);
                        self.array[j].parent = None;
                    }
                }
            },
            (Some(j), None) => {
                match self.array[last].parent {
                    Some(i) => {
                        if self.array[last].value < self.array[i].value {
                            self.array[i].left = Some(j);
                        } else {
                            self.array[i].right = Some(j);
                        }
                        self.array[j].parent = Some(i);
                    },
                    None => {
                        self.root = Some(j);
                        self.array[j].parent = None;
                    }
                }
            },
            (Some(j1), Some(j2)) => {

                let sucessor = self.sucessor(last).unwrap();

                self.array[j1].parent = Some(sucessor);
                self.array[sucessor].left = Some(j1);

                if sucessor == j2 {

                    match self.array[last].parent {
                        Some(i) => {
                            if self.array[last].value < self.array[i].value {
                                self.array[i].left = Some(sucessor);
                            } else {
                                self.array[i].right = Some(sucessor);
                            }
                            self.array[sucessor].parent = Some(i);
                        },
                        None => {
                            self.root = Some(sucessor);
                            self.array[sucessor].parent = None;
                        }
                    }
                } else {

                    self.array[j2].parent = Some(sucessor);

                    let filho_sucessor = self.array[sucessor].right;
                    self.array[sucessor].right = Some(j2);

                    let pai_sucessor = self.array[sucessor].parent.unwrap();

                    match filho_sucessor {
                        Some(k) => {
                            self.array[pai_sucessor].left = Some(k);
                            self.array[k].parent = Some(pai_sucessor)
                        }
                        None => {
                            self.array[pai_sucessor].left = None;
                        }
                    }
                }
            }
        }
        Some(self.array.pop().unwrap().satelite)
    }

    fn minimum_aux(self: &Self, mut index: usize) -> usize {

        loop {
            match self.array[index].left {
                Some(j) => {
                    index = j;
                    continue;
                }
                None => {
                    break index
                }
            }
        }
    }

    fn maximum_aux(self: &Self, mut index: usize) -> usize {

        loop {
            match self.array[index].right {
                Some(j) => {
                    index = j;
                    continue;
                }
                None => {
                    break index
                }
            }
        }
    }


    fn sucessor(self: &Self, index: usize) -> Option<usize> {

        match self.array[index].right {

            Some(i) => Some(self.minimum_aux(i)),

            None => {
                let mut pai = self.array[index].parent;
                loop {
                    match pai {
                        Some(j) => {
                            if self.array[j].value > self.array[index].value {   // se for igual não é sucessor, é predecessor
                                return Some(j)                              // o primeiro maior nessa busca éo sucessor
                            } else {
                                pai = self.array[j].parent;
                                continue;
                            }
                        },
                        None => return None
                    }
                }
            }
        }
    }

    fn predecessor(self: &Self, index: usize) -> Option<usize> {

        match self.array[index].left {

            Some(i) => Some(self.maximum_aux(i)),

            None => {
                let mut pai = self.array[index].parent;
                loop {
                    match pai {
                        Some(j) => {
                            if self.array[j].value <= self.array[index].value {
                                return Some(j)
                            } else {
                                pai = self.array[j].parent;
                                continue;
                            }
                        },
                        None => return None
                    }
                }
            }
        }
    }

    pub fn get_sucessor(self: &Self, value: &T) -> Option<(&T, &E)> {

        let opt_index = self.get_index(value);

        match opt_index {

            Some(index) => {
                match self.sucessor(index) {
                    Some(i) => Some((&self.array[i].value, &self.array[i].satelite)),
                    None => None
                }
            }
            None => {
                println!("Valor procurado não existe na árvore.");
                None
            }
        }


    }

    pub fn get_predecessor(self: &Self, value: &T) -> Option<(&T, &E)> {

        let opt_index = self.get_index(value);

        match opt_index {

            Some(index) => {
                match self.predecessor(index) {
                    Some(i) => Some((&self.array[i].value, &self.array[i].satelite)),
                    None => None
                }
            }
            None => {
                println!("Valor procurado não existe na árvore.");
                None
            }
        }
    }

}

