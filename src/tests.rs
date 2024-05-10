use super::BinarySearchTree;

#[test]
fn corret_insertion_test() {

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

    assert_eq!(b.inorder(), vec![(&2, &"D"), (&2, &"L"), (&3, &"C"), (&4, &"E"), (&6, &"B"), (&7, &"F"), (&9, &"H"),
                                 (&13, &"G"), (&15, &"A"), (&17, &"J"), (&18, &"I"), (&18, &"M"), (&20, &"K")]);


    assert_eq!(b.get_sucessor(&2), Some((&2, &"L")));
    assert_eq!(b.get_sucessor(&3), Some((&4, &"E")));
    assert_eq!(b.get_sucessor(&4), Some((&6, &"B")));
    assert_eq!(b.get_sucessor(&6), Some((&7, &"F")));
    assert_eq!(b.get_sucessor(&7), Some((&9, &"H")));
    assert_eq!(b.get_sucessor(&9), Some((&13, &"G")));
    assert_eq!(b.get_sucessor(&13), Some((&15, &"A")));
    assert_eq!(b.get_sucessor(&15), Some((&17, &"J")));
    assert_eq!(b.get_sucessor(&17), Some((&18, &"I")));
    assert_eq!(b.get_sucessor(&18), Some((&18, &"M")));
    assert_eq!(b.get_sucessor(&20), None);

    assert_eq!(b.get_predecessor(&2), None);
    assert_eq!(b.get_predecessor(&3), Some((&2, &"L")));
    assert_eq!(b.get_predecessor(&4), Some((&3, &"C")));
    assert_eq!(b.get_predecessor(&6), Some((&4, &"E")));
    assert_eq!(b.get_predecessor(&7), Some((&6, &"B")));
    assert_eq!(b.get_predecessor(&9), Some((&7, &"F")));
    assert_eq!(b.get_predecessor(&13), Some((&9, &"H")));
    assert_eq!(b.get_predecessor(&15), Some((&13, &"G")));
    assert_eq!(b.get_predecessor(&17), Some((&15, &"A")));
    assert_eq!(b.get_predecessor(&18), Some((&17, &"J")));
    assert_eq!(b.get_predecessor(&20), Some((&18, &"M")));

}