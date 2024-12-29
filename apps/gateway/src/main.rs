fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    apollo_router::main()
}
