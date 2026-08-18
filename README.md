# rinha-backend-rust — Rinha de Backend (Rust + Actix-web)

Implementação do desafio **Rinha de Backend 2024** em Rust usando Actix-web. API de alta performance para gerenciamento de pessoas com validação, busca e contagem.

## 📚 Sobre
Submissão para a **Rinha de Backend** — competição de performance de APIs em Rust. Requisitos:
- `POST /pessoas` — Criar pessoa (validação: apelido único, nome ≤ 100, apelido ≤ 32, nascimento ISO 8601, stack opcional ≤ 32 chars each)
- `GET /pessoas/:id` — Buscar por ID
- `GET /pessoas?t=` — Buscar por termo (nome, apelido, stack)
- `GET /contagem-pessoas` — Contar total

## 🛠 Tech Stack
- **Language**: Rust 2021 edition
- **Framework**: Actix-web 4
- **Serialization**: Serde + serde_json
- **Runtime**: Tokio (async)

## 🚀 Como rodar
```bash
# Instale Rust: https://rustup.rs/
cargo build --release
cargo run --release

# Server roda em http://127.0.0.1:3000
```

## 📁 Estrutura
```
src/
├── main.rs           # Entry point + server config
├── controllers/
│   ├── mod.rs        # Router
│   └── person.rs     # Handlers (POST, GET, search, count)
Cargo.toml
LICENSE
```

## 🔧 Endpoints
| Método | Rota | Descrição |
|--------|------|-----------|
| POST | `/pessoas` | Criar pessoa |
| GET | `/pessoas/:id` | Buscar por UUID |
| GET | `/pessoas?t=termo` | Buscar por termo (ILIKE) |
| GET | `/contagem-pessoas` | Contar total |

## 📊 Performance
- Actix-web (actor model, zero-copy)
- Validação inline sem alocações desnecessárias
- Preparado para load testing (wrk, vegeta)

## 📄 Licença
MIT — veja [LICENSE](LICENSE).