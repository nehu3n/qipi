<img src="./public/logo.png" width="150px" align="right" />

> [!WARNING]
> **Qipi** is in the development stage. It is not ready for any use at this time. ⏰

# Qipi - Package Manager

🦉 **Qipi** is a **blazing-fast**, **disk-efficient**, and **deterministic** package manager for NodeJS. **Written in Rust.**

## ✨ Features

- ⚡ **Blazing-fast:** All fetching, downloading, unzipping, and linking operations are performed concurrently, making Qipi the fastest package manager for NodeJS. **Up to 100 times faster than NPM!** [(see benchmarks)](./tests/benchmarks/)

- 💾 **Disk-efficient:** All packages are stored in a global cache, preventing duplicate downloads. For each project, a symlink to its cached location is created. Intelligent algorithms clean up unused packages.

- 🔒 **Secure:** Cryptographic signatures of packages are verified before installation, preventing interception or fraudulent downloads.

- ⛄ **Deterministic:** For security reasons, packages are stored in frozen versions to avoid conflicts between different versions. Each new installation uses the same version previously installed, ensuring consistency.

- 📂 **Available registries:** NPM and JSR registries are supported, with **NPM set as the default.**

## 🚀 Getting Started

📥 **Installation:**

To install Qipi, use the following command:

```bash
npm install -g qipi
```

🌷 **Basic Usage:**

To add a package:

```bash
qp add <package-name>
```

To remove a package:

```bash
qp remove <package-name>
```

To install all packages for a project:
```bash
qp install
```

🧩 **Configuration:**

You can configure Qipi by editing the `~/.qipi/config.toml` file.

## 📈 Benchmarks

**Qipi** is the _fastest package manager_. You can see the different benchmarks [here](./tests/benchmarks).

## 📚 Documentation

For detailed documentation, visit the [Qipi Documentation](https://github.com/nehu3n/qipi/wiki).

## 🤝 Contributing

Contributions are welcome! Please see our [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines on how to contribute.

## 📄 License

**Qipi** is licensed under the [MIT License](./LICENSE).


<hr />

<div align="right">

##### Thanks to [Camilo Zuluaga](https://github.com/camilo-zuluaga) for creating the logo. ❤

</div>
