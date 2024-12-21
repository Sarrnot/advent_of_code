use std::{collections::VecDeque, fs};

const FILE_PATH: &str = "./resources/2024_09.txt";

/// Solution for https://adventofcode.com/2024/day/9 - Part One & Two.
/// Run by `cargo run --bin 2024_09`.
fn main() {
    // Read and parse file
    let file_content = fs::read_to_string(FILE_PATH).unwrap();
    let (disk, fragments) = parse_file(&file_content);

    // Calculate
    let disk_dfrg_by_block = defragment_by_block(disk.clone());
    let disk_dfrg_by_file = defragment_by_file(disk.clone(), fragments);

    // Print result
    println!(
        "Checksum - defragmented by block: {}",
        get_checksum(&disk_dfrg_by_block)
    );
    println!(
        "Checksum - defragmented by file: {}",
        get_checksum(&disk_dfrg_by_file)
    );
}

fn defragment_by_block(mut disk: Disk) -> Disk {
    // Create a list of free space indexes
    let mut free_space_list = VecDeque::new();

    for (i, block) in (&disk).into_iter().enumerate() {
        match block {
            DiskBlock::Free => free_space_list.push_back(i),
            DiskBlock::File(_) => (),
        }
    }

    // Prepare all moves
    struct Move {
        from: usize,
        to: usize,
    }
    let mut move_buffer: Vec<Move> = vec![];
    for (block_index, block) in (&disk).into_iter().enumerate().rev() {
        if free_space_list.len() == 0 {
            break;
        }

        match block {
            DiskBlock::Free => continue,
            DiskBlock::File(_) => {
                let free_index = free_space_list.pop_front().unwrap();
                if free_index > block_index {
                    break;
                }
                move_buffer.push(Move {
                    from: block_index,
                    to: free_index,
                });
            }
        }
    }

    // Execute all prepared moves
    for move_action in move_buffer {
        let block = std::mem::replace(&mut disk[move_action.from], DiskBlock::Free);
        disk[move_action.to] = block;
    }

    disk
}

fn defragment_by_file(mut disk: Disk, fragments: Vec<Fragment>) -> Disk {
    // Create lists of fragments
    let mut file_fragments: Vec<Fragment> = vec![];
    let mut free_fragments: Vec<Fragment> = vec![];

    for fragment in fragments {
        match fragment.block_type {
            DiskBlock::File(_) => file_fragments.push(fragment),
            DiskBlock::Free => free_fragments.push(fragment),
        }
    }

    // Iterate files from back
    for file_fragment in (&mut file_fragments).into_iter().rev() {
        if free_fragments.len() == 0 {
            break;
        }

        // Move to first free space where the file can fit
        for free_fragment in &mut free_fragments {
            if free_fragment.start_index > file_fragment.start_index {
                break;
            }

            if free_fragment.length < file_fragment.length {
                continue;
            }

            // Move individual blocks
            for i in 0..file_fragment.length {
                let block =
                    std::mem::replace(&mut disk[file_fragment.start_index + i], DiskBlock::Free);
                disk[free_fragment.start_index + i] = block;
            }

            // Update fragments
            file_fragment.start_index = free_fragment.start_index;
            free_fragment.start_index += file_fragment.length;
            free_fragment.length -= file_fragment.length;

            break;
        }
    }

    disk
}

fn get_checksum(disk: &Disk) -> usize {
    let mut checksum = 0;

    for (i, block) in (&disk).into_iter().enumerate() {
        match block {
            DiskBlock::File(id) => checksum += id * i,
            DiskBlock::Free => (),
        }
    }

    checksum
}

fn parse_file(content: &str) -> (Disk, Vec<Fragment>) {
    let mut disk: Disk = vec![];
    let mut fragments: Vec<Fragment> = vec![];

    let mut is_file = true;
    let mut file_id = 0;
    let mut start_index = 0;
    for character in content.chars() {
        let count = character.to_digit(10).expect("Not a number.") as usize;

        let block = match is_file {
            true => DiskBlock::File(file_id),
            false => DiskBlock::Free,
        };

        for _ in 0..count {
            disk.push(block.clone());
        }

        if count > 0 {
            fragments.push(Fragment {
                block_type: block,
                length: count,
                start_index,
            });
        }

        if is_file {
            file_id += 1;
        }
        is_file = !is_file;
        start_index += count;
    }

    (disk, fragments)
}

type Disk = Vec<DiskBlock>;
type FileId = usize;

#[derive(Clone)]
enum DiskBlock {
    File(FileId),
    Free,
}
struct Fragment {
    block_type: DiskBlock,
    length: usize,
    start_index: usize,
}
