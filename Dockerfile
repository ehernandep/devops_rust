FROM rust:latest 

WORKDIR /usr/src/soccer

ENV DATABASE_URL=postgresql://ehernandep:Tonterias4316*@finalproject.csif2sx3kogz.us-east-1.rds.amazonaws.com:5432/finalproject

COPY . . 

RUN cargo build --release

CMD cargo run --release
