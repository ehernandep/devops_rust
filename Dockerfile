FROM rust:1.73.0

WORKDIR /usr/src/

COPY . .

ENV DATABASE_URL=postgresql://ehernandep:Tonterias4316*@finalproject.csif2sx3kogz.us-east-1.rds.amazonaws.com:5432/finalproject

RUN cargo build --release

EXPOSE 8080

CMD ["cargo", "run", "--release"]
