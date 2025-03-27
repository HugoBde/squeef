pub fn parse_command(input: &str) -> Result<Vec<Command>, ParseError> {
    for line in input.lines() {
        let cmd = parse_command_line(line)?;
        cmds.push(cmd);
    }
    Ok(cmds)
}
