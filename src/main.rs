use std::{collections::{HashMap, BinaryHeap}};

#[derive(Debug, Eq, PartialEq, Clone)]
struct Node {
    frecuency: u8,
    character: Option<char>,
    node_left: Option<Box<Node>>,
    node_right: Option<Box<Node>>
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.frecuency.cmp(&other.frecuency)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cmp(self))
    }
}

fn calculate_frecuency(_str: &str) -> HashMap<char, u8> {
    let mut frecuency_map: HashMap<char, u8> = HashMap::new();
    
    _str.chars().all(|c: char| {
        let counter = frecuency_map.entry(c).or_insert(0);
        *counter += 1;

        true
    });
    
    frecuency_map
}

fn build_huffman_tree(frecuency_map: & HashMap<char, u8>) -> BinaryHeap<Node> {
    let mut binary_heap: BinaryHeap<Node> = BinaryHeap::new();
    frecuency_map.into_iter().all(|key_nd_value| {
        let character = key_nd_value.0;
        let frecuency = key_nd_value.1;

        binary_heap.push(Node{
            frecuency: frecuency.to_owned(),
            character: Some(character.to_owned()),
            node_left: None,
            node_right: None});

        true
    });
    

    while binary_heap.len() > 1 {
        let node_left = binary_heap.pop().unwrap();
        let node_right = binary_heap.pop().unwrap();

        binary_heap.push(Node{
            character: None,
            frecuency: node_left.frecuency + node_right.frecuency,
            node_left: Some(Box::new(node_left)),
            node_right: Some(Box::new(node_right))
        });
    }

    binary_heap
}

fn _encode<'a>(node: Option<&Node>, b: & 'a str, map: & mut HashMap<char, & 'a str>)  {
    if node == None {
        return
    }

    let root_node = node.unwrap();

    let node_left = root_node.node_left.as_ref();
    let node_right = root_node.node_right.as_ref();

    if node_left == None && node_right == None {
        if b.len() > 1 {
            map.insert(root_node.character.unwrap(), b);
        } else {
            map.insert(root_node.character.unwrap(), "1");
        }

        return
    }


    _encode(Some(root_node.node_left.as_deref().unwrap()), Box::leak(format!("{}0", b).into_boxed_str()), map);
    _encode(Some(root_node.node_right.as_deref().unwrap()), Box::leak(format!("{}1", b).into_boxed_str()), map);
}


fn encode(text: &str) -> (String, Node) {
    let mut frecuency_map = calculate_frecuency(text);
    let huffman_tree = build_huffman_tree(&frecuency_map);
    frecuency_map.clear();
    
    let root_node = huffman_tree.peek().unwrap();

    let mut char_to_bits: HashMap<char, &str> = HashMap::new();
    _encode(Some(root_node), "", & mut char_to_bits);

    let mut res = String::new();

    text.chars().all(|v| {
        res.push_str(char_to_bits.get(&v).unwrap());
        
        true
    });

    (res, root_node.to_owned())
}

fn _decode(node: Option<&Node>, i: i64, binary_str: &str) -> i64 {
    if node == None {
        return i
    }

    let root_node = node.unwrap();
    let node_left = root_node.node_left.as_ref();
    let node_right = root_node.node_right.as_ref();

    if node_left == None && node_right == None {
        print!("{}", root_node.character.unwrap());
        return i
    }

    let index = i +1;
    let node = if binary_str.chars().nth(index as usize).unwrap() == '0' { node_left } else { node_right };
    return _decode(Some(node.as_deref().unwrap()), index, binary_str)
}

fn decode(binary_str: &str, node: Option<&Node>) {
    let mut index: i64 = -1;
    while index < binary_str.len() as i64 - 1 {
        index = _decode(node, index, binary_str);
    }
}

fn main() {
    let example_string = "Huffman coding is a data compression algorithm.";
    let (compressed_string, root_node) = encode(example_string);
    
    println!("compressed string: {:?}\n", compressed_string);

    decode(&compressed_string, Some(&root_node));
}
