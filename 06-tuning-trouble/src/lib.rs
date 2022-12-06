use std::collections::HashSet;

pub fn marker_start(packet: &str, length: usize) -> u32 {
    let mut index = 0;
    for (i, window) in packet.as_bytes().windows(length).enumerate() {
        if HashSet::<&u8>::from_iter(window.iter()).len() == length {
            index = (i + length) as u32;
            break;
        }
    }

    index
}

pub fn packet_start(packet: &str) -> u32 {
    marker_start(packet, 4)
}

pub fn message_start(packet: &str) -> u32 {
    marker_start(packet, 14)
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

    #[test]
    fn message_start_1() {
        assert_eq!(message_start("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23)
    }

    #[test]
    fn message_start_2() {
        assert_eq!(message_start("nppdvjthqldpwncqszvftbrmjlhg"), 23)
    }

    #[test]
    fn message_start_3() {
        assert_eq!(message_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29)
    }

    #[test]
    fn message_start_4() {
        assert_eq!(message_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26)
    }
}
