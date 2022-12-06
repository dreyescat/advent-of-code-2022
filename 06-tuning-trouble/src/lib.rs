use std::collections::HashSet;

pub fn packet_start(packet: &str) -> u32 {
    let mut index = 0;
    for (i, window) in packet.as_bytes().windows(4).enumerate() {
        if HashSet::<&u8>::from_iter(window.iter()).len() == 4 {
            index = (i + 4) as u32;
            break;
        }
    }

    index
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn packet_start_1() {
        assert_eq!(packet_start("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5)
    }

    #[test]
    fn packet_start_2() {
        assert_eq!(packet_start("nppdvjthqldpwncqszvftbrmjlhg"), 6)
    }

    #[test]
    fn packet_start_3() {
        assert_eq!(packet_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10)
    }

    #[test]
    fn packet_start_4() {
        assert_eq!(packet_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11)
    }
}
