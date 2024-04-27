/// -h/--help
struct Help;

impl Flag for Help {
    fn name_short(&self) -> Option<u8> {
        Some(b'h')
    }
    fn name_long(&self) -> &'static str {
        "help"
    }
    fn desc(&self) -> &'static str {
        "This flag prints the help output"
    }
    fn update(&self, value: String, args: &mut Args) {
        Ok(())
    }
}
