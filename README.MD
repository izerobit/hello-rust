docker build -t hello-rust -f ./Dockerfile .
docker run -p 8000:8000 hello-rust
visit localhost:8000/health
