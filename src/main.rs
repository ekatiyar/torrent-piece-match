use bip_metainfo::Metainfo;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Ensure that the user provides exactly two arguments
    if args.len() != 3 {
        eprintln!("Usage: {} <source torrent_file> <destination torrent_file>", args[0]);
        std::process::exit(1);
    }

    // Extract the paths to the torrent files from the command-line arguments
    let source_torrent_file = &args[1];
    let dest_torrent_file = &args[2];

    // Parse torrent files
    let source_torrent_info = parse_torrent(source_torrent_file);
    let dest_torrent_info = parse_torrent(dest_torrent_file);

    // Compare piece hash data
    compare_piece_hashes(&source_torrent_info, &dest_torrent_info);
}

fn parse_torrent(file_path: &str) -> Metainfo {
    let path = Path::new(file_path);
    let mut file = File::open(&path).expect("Unable to open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Unable to read data");

    Metainfo::from_bytes(&data).expect("Unable to parse torrent file")
}

fn compare_piece_hashes(source_torrent_info: &Metainfo, dest_torrent_info: &Metainfo) {
    let source_pieces = source_torrent_info.info().pieces();
    let dest_pieces = dest_torrent_info.info().pieces();
    let mut piece_map: HashMap<&[u8], u32> = HashMap::new();

    // populate hashmap with pieces from source torrent
    for hash in source_pieces
    {
        *piece_map.entry(hash).or_insert(0) += 1;
    }

    let mut matches = 0;
    // compare with pieces from dest torrent;
    for (i, hash) in dest_pieces.enumerate()
    {
        if piece_map.contains_key(hash)
        {
            println!("Matching Piece: {} : {:?}", i, hex::encode(hash));
            matches += 1;
        }
    }

    println!("Comparison complete. Found {} matches", matches);
}