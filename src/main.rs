use mrg::compress;

fn main() {
    let data = (0..100000)
        .map(|_| b"oh what a beautiful morning, oh what a beautiful day!!".to_vec())
        .flat_map(|v| v)
        .collect::<Vec<u8>>();
    let compressed = compress(&data);
}