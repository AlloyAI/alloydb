# AlloyDB: Lightweight Vector Database

AlloyDB is a lightweight, file-based vector database designed to be an alternative to server-based vector databases such as Weavite, Pinecone, and others. It's tailored for smaller projects where the need for a lightweight and performant solution is paramount.

## 🌟 Features

- **Lightweight**: No need for hefty server installations or configurations.
- **File-based**: Just like SQLite, everything is managed within a single file.
- **Fast**: Optimized for speed and efficiency.
- **Easy Integration**: Designed for easy integration into small to medium-sized projects.

## 🚀 Getting Started

### Installation

`cargo add alloydb`

### Basic Usage

       use alloydb;
       use anyhow::Result;

       fn main() -> Result<()> {
       // Init a new AlloyDB
       let mut db = alloydb::Db::new("db.alloy".to_string());

       // Insert data into the db
       db.insert("Hello AlloyDB!")?;
    }

## 💡 Use Cases

- **Prototype Development**: Quickly incorporate vector search capabilities in your prototypes.
- **Embedded Systems**: Ideal for systems where resources are limited.
- **Offline Applications**: Use AlloyDB for applications that need to run offline or on local networks.

## 📜 License

AlloyDB is licensed under the [MIT License](https://github.com/claytoncasey01/alloydb/blob/dev/LICENSE).

## 🤖 Authors

- Casey Clayton ([@claytoncasey01](https://github.com/claytoncasey01))
