pub fn hunter_request(request: [u8; 3]) -> [u8; 3] {
    // Native kind 2 is an AI companion; controlled Prowlers use player kind 1.
    if request[0] == 2 { [1, 0, 255] } else { request }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn keeps_players_coordinates_and_absent_targets() {
        for kind in [0, 1, 3, 4, 5, 6, 255] {
            for index in [0, 1, 2, 3, 255] {
                assert_eq!(hunter_request([kind, 2, index]), [kind, 2, index]);
            }
        }
        for mode in [0, 2, 10, 255] {
            for index in [0, 1, 2, 3, 255] {
                assert_eq!(hunter_request([2, mode, index]), [1, 0, 255]);
            }
        }
    }
}
