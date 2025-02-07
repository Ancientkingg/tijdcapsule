cd client
npm ci
npm run deploy
cd ..

cd server
cargo build --release
cargo run --bin server
cd ..