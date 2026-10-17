pub fn add_command(
    history: &mut std::collections::VecDeque<String>,
    command: String,
    k: usize
) {
    let length: usize = history.len();
    if k == 0 {
        return;
    }
    if length + 1 > k {
        history.pop_front();
        history.push_back(command);
    }
    else {
        history.push_back(command);
    }
}
