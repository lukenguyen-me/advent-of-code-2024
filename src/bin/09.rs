advent_of_code::solution!(9);

struct Disk {
    files: Vec<String>,
}

impl Disk {
    fn new(input: &str) -> Self {
        let mut files = vec![];
        let mut id: u64 = 0;

        for (i, c) in input.trim().chars().enumerate() {
            if let Ok(count) = c.to_string().parse::<u64>() {
                for _ in 0..count {
                    if i % 2 == 0 {
                        files.push(id.to_string());
                    } else {
                        files.push(".".to_string());
                    }
                }
                if i % 2 == 0 {
                    id += 1;
                }
            } else {
                println!("{:?}", c);
            }
        }

        Disk { files }
    }

    fn optimize(&mut self) {
        let mut i = 0;
        let mut j = self.files.len() - 1;

        while i < j {
            if self.files[i] == "." {
                while self.files[j] == "." {
                    j -= 1;
                }
                self.files.swap(i, j);
                i += 1;
                j -= 1;
            } else {
                i += 1;
            }
        }
    }

    fn find_slot(&self, j: usize) -> (Option<usize>, usize) {
        let c = self.files[j].clone();
        let mut x = 0;
        let mut count: usize = 0;
        let mut count_empty: usize = 0;
        while count <= j && self.files[j - count] == c {
            count += 1;
        }
        while count_empty < count && x < j {
            if self.files[x] == "." {
                count_empty += 1;
            } else {
                count_empty = 0;
            }
            x += 1;
        }

        if count_empty == count {
            (Some(x - count), count)
        } else {
            (None, count)
        }
    }

    fn optimize_2(&mut self) {
        let mut j = self.files.len() - 1;

        loop {
            while self.files[j] == "." {
                j -= 1;
            }
            let (slot, count) = self.find_slot(j);
            if let Some(slot) = slot {
                for z in 0..count {
                    self.files.swap(slot + z, j - z);
                }
            }
            if j > count {
                j -= count;
            } else {
                break;
            }
        }
    }

    fn get_checksum(&self) -> u64 {
        let mut sum = 0;
        for (i, c) in self.files.iter().enumerate() {
            if let Some(num) = c.parse::<u64>().ok() {
                sum += num * i as u64;
            }
        }
        sum
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk = Disk::new(input);
    disk.optimize();

    Some(disk.get_checksum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk = Disk::new(input);
    disk.optimize_2();

    Some(disk.get_checksum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
