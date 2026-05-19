fn main() -> anyhow::Result<()> {
    env_logger::init();
    plc_lsp::run()
}
