#[derive(Debug, Eq, PartialEq)]
struct Node {
    freq: u32,
    char: Option<char>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq) 
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn generate_frequency_table(data: &str) -> HashMap<char, u32> {
    let mut freq_table = HashMap::new();
    for ch in data.chars() {
        *freq_table.entry(ch).or_insert(0) += 1;
    }
    freq_table
}

fn par_generate_frequency_table(data: &str) -> HashMap<char, u32> {
    let freq_table = data
        .par_chars()
        .fold(HashMap::new, |mut table, letter| {
            *table.entry(letter)
                .or_insert(0) += 1;
            table
        })
        .reduce(HashMap::new, |mut left_table, right_table| {
            for (letter, letter_freq) in right_table {
                *left_table.entry(letter)
                    .or_insert(0) += letter_freq;
            }

            left_table
        });

    freq_table
}

fn build_huffman_tree(freq_table: &HashMap<char, u32>) -> Option<Node> {
    let mut heap: BinaryHeap<Node> = freq_table
        .par_iter()
        .map(|(&ch, &freq)| Node {
            freq,
            char: Some(ch),
            left: None,
            right: None,
        })
        .collect();

    while heap.len() > 1 {
        let left = heap.pop()
                            .unwrap();

        let right = heap.pop()
                            .unwrap();

        let merged = Node {
            freq: left.freq + right.freq,
            char: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };

        heap.push(merged);
    }

    heap.pop()
}

fn generate_codes(node: &Node, prefix: String, codes: &mut HashMap<char, String>) {
    if let Some(ch) = node.char {
        codes.insert(ch, prefix);
    } else {
        if let Some(ref left) = node.left {
            generate_codes(left, format!("{}0", prefix), codes);
        }
        if let Some(ref right) = node.right {
            generate_codes(right, format!("{}1", prefix), codes);
        }
    }
}

fn huffman_encoding(data: &str) -> (HashMap<char, String>, String) {
    let freq_table = generate_frequency_table(data);
    let huffman_tree = build_huffman_tree(&freq_table).unwrap();

    let mut codes = HashMap::new();
    generate_codes(&huffman_tree, String::new(), &mut codes);

    let encoded_data: String = data
        .chars()
        .par_bridge()
        .map(|ch| codes[&ch].clone())
        .collect();

    (codes, encoded_data)
}