use BinarySearchTrees::BinarySearchTree;

fn main() {

    let mut b = BinarySearchTree::new();

    b.insert(5, "A");
    b.insert(8, "G");
    b.insert(0, "X");
    b.insert(2, "Z");
    b.insert(2, "W");
    b.insert(5, "H");

    let w = b.inorder();

    println!("{:?}", w);
    println!("{:?}", b.get(&2));
    println!("{:?}", b.get(&11));
    println!("{:?}", b.minimum());
    println!("{:?}", b.maximum());


    //-------------------

    let mut b = BinarySearchTree::new();

    b.insert(15, "A");
    b.insert(6, "B");
    b.insert(3, "C");
    b.insert(2, "D");
    b.insert(4, "E");
    b.insert(7, "F");
    b.insert(13, "G");
    b.insert(9, "H");
    b.insert(18, "I");
    b.insert(17, "J");
    b.insert(20, "K");
    b.insert(2, "L");
    b.insert(18, "M");

    println!("{:?}", b.inorder());
    println!("Deletion");

    b.deletion(&15);
    println!("{:?}", b.inorder());


}
