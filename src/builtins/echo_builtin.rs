use std::collections::VecDeque;


pub(crate) fn echo_builtin(inputs: &mut VecDeque<&str>) -> String {
    let output = if let Some(output) = inputs.pop_front() { output } else { "" };
    format!("{}\n", output)
}
