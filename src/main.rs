use trie::Trie;

fn main() {
    let mut trie = Trie::new();
    trie.insert("f");
    trie.delete("f");

    trie.insert("foo");
    trie.insert("fool");
    trie.insert("fox");
    trie.insert("foolish");
    // trie.insert("foolick");
    trie.insert("bar");
    trie.disp();

    println!("{:?}", trie.find("fo"));
}
