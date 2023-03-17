FROM rust:slim
COPY ./target/release/secure-data-exchange ./target/release/secure-data-exchange
COPY ./wwwroot ./wwwroot 
ENTRYPOINT ["./target/release/secure-data-exchange"]