# TeaPOS

A modern, blazingly fast Point-of-Sale system for restaurants

## 🚧 Project Status
This project is under active development and is not yet feature complete. Contributions and feedback are welcome!

## 🛠️ Tech Stack
- **Backend:** Rust (with ntex.rs for HTTP Server and diesel for ORM)
- **Database:** PostgreSQL
- **Frontend:** Vite + React + TypeScript + TailwindCSS v4

## 📦 Setup & Installation

### 1. Prerequisites
- Rust (latest stable)
- PostgreSQL
- pnpm

### 2. Clone the Repository
```bash
git clone https://github.com/your-username/TeaPOS.git
cd TeaPOS
```

### 3. Configure Environment
Copy the example environment file and edit as needed:
```bash
cp .env.example .env
```
Update the values in `.env` to match your local setup:
```
SERVER_ADDRESS=127.0.0.1
SERVER_PORT=3000
DATABASE_URL=postgres://teapos:123456@localhost:5432/teapos
DATABASE_POOL_SIZE=10
```

### 4. Install Frontend Dependencies
```bash
pnpm install
```

### 5. Run Database Migrations
```bash
cargo install diesel_cli --no-default-features --features postgres
# Then run the migraton:
diesel migration run
```

### 6. Run the Application
```bash
# run the backend (debug):
cargo run
# and then, run the vite development server for the frontend:
pnpm run dev
```

## ✨ Features (Planned & Current)
- [ ] User authentication & authorization
- [ ] Menu, product, and inventory management
- [ ] Order and sales tracking
- [ ] Table and seat management
- [ ] Web API endpoints
- [ ] Frontend interface

## 🗺️ Roadmap
- [ ] Complete core backend APIs
- [ ] Implement authentication
- [ ] Add product/inventory/order modules
- [ ] Build a modern frontend UI
- [ ] Write tests and documentation

## 🤝 Contributing
Pull requests and issues are welcome! Please open an issue to discuss any major changes.

## 📄 License
This project is licensed under the MIT License.
