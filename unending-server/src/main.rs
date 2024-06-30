mod llm;
mod log;

fn main() {
    log::info(
        &format!(
            "{:?}",
            llm::gpt4all_chat("Write a simple test message and respond with only that.", 200),
        )
    );
}
