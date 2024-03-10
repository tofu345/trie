struct Node {
    value: char,
    children: Vec<Link>,
    is_word: bool,
}

type Link = *mut Node;

pub struct Trie {
    head: Link,
}

impl Node {
    fn new(value: char) -> Self {
        Node {
            value,
            children: Vec::new(),
            is_word: false,
        }
    }
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            head: Box::into_raw(Box::new(Node::new('\0'))),
        }
    }

    pub fn disp(&self) {
        fn walk(curr: Link) {
            unsafe {
                println!("{} {}", (*curr).value, (*curr).is_word);
                for &node in (*curr).children.iter() {
                    walk(node);
                }
            }
        }

        walk(self.head);
    }

    pub fn insert(&mut self, value: &str) {
        fn recurse(curr: Link, val: &[char]) {
            if val.is_empty() {
                return;
            }

            unsafe {
                let node = match (*curr)
                    .children
                    .iter()
                    .find(|&&v| v.as_ref().unwrap().value == val[0])
                {
                    Some(v) => *v,
                    None => {
                        let n = Box::into_raw(Box::new(Node::new(val[0])));
                        (*curr).children.push(n);
                        n
                    }
                };

                if val.len() == 1 {
                    (*node).is_word = true;
                }

                recurse(node, &val[1..]);
            }
        }

        recurse(self.head, &value.chars().collect::<Vec<char>>());
    }

    pub fn find(&self, partial: &str) -> Vec<String> {
        fn recurse(curr: Link, partial: &mut String, words: &mut Vec<String>) {
            if partial.is_empty() {
                return;
            }

            unsafe {
                if (*curr).value != '\0' {
                    partial.push((*curr).value);
                }

                for &node in (*curr).children.iter() {
                    if (*node).is_word {
                        words.push(format!("{}{}", partial, (*node).value));
                    }

                    recurse(node, partial, words);
                }
            }
        }

        let mut words = Vec::new();
        let mut curr = self.head;
        for ch in partial.chars() {
            unsafe {
                curr = match (*curr).children.iter().find(|&&v| (*v).value == ch) {
                    Some(v) => *v,
                    None => return words,
                };
            }
        }

        let mut partial = partial.to_owned();
        partial.pop();
        recurse(curr, &mut partial.to_owned(), &mut words);
        words
    }

    pub fn delete(&mut self, value: &str) {
        fn recurse(curr: Link, value: &[char]) {
            if value.is_empty() {
                return;
            }

            unsafe {
                let next = match (*curr).children.iter().find(|&&v| (*v).value == value[0]) {
                    None => return,
                    Some(v) => *v,
                };

                if value.len() == 1 {
                    if (*next).children.is_empty() {
                        let _ = Box::from_raw(next);
                        return;
                    }

                    (*next).is_word = false;
                }

                recurse(next, &value[1..]);
            }
        }

        recurse(self.head, &value.chars().collect::<Vec<char>>());
    }
}

impl Drop for Trie {
    fn drop(&mut self) {
        fn recurse(curr: Link) {
            unsafe {
                let curr: Box<Node> = Box::from_raw(curr);
                for node in curr.children {
                    recurse(node);
                }
            }
        }

        recurse(self.head);
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_vecs(v1: Vec<String>, v2: Vec<&str>) {
        // println!("{:?} {:?}", v1, v2);
        assert!(v2.into_iter().all(|v| v1.contains(&v.to_owned())))
    }

    #[test]
    fn test() {
        let mut trie = Trie::new();
        trie.insert("foo");
        trie.insert("fool");
        trie.insert("foolish");
        trie.insert("bar");

        trie.disp();

        assert_vecs(trie.find("fo"), vec!["foo", "fool", "foolish"]);

        trie.delete("fool");

        trie.disp();

        assert_vecs(trie.find("fo"), vec!["foo", "foolish"]);
    }
}
