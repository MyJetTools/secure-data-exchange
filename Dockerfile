FROM rust:slim
COPY ./target/release/secure-data-exchange ./target/release/secure-data-exchange
ENTRYPOINT ["./target/release/secure-data-exchange"]