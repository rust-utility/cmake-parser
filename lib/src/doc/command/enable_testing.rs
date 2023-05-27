#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn enable_testing() {
        let src = include_bytes!("../../../../fixture/commands/enable_testing");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);

        assert_eq!(doc.commands(), Ok(vec![Command::EnableTesting,]))
    }
}
