# 🚀 Cryptocurrency Info Aggregator  
### 📚 Blockchain Technologies 2 — Assignment 1.4

A simple and powerful web app built in **Rust** using the **Actix Web framework** that allows users to search for cryptocurrencies by symbol (like `BTC`, `ETH`, etc.) and fetch the latest data from the **CoinGecko API**.

---

## 🔧 Features

✅ No API Key required — uses public CoinGecko API  
✅ Built entirely in Rust  
✅ Web interface for user interaction  
✅ Displays:

- 🪙 Name of the cryptocurrency  
- 🖼️ Logo  
- 📄 Short description  
- 🔗 Official homepage  
- 🕒 Last updated time  

---

## 🛠️ Tech Stack

| Layer     | Technology         |
|-----------|--------------------|
| Backend   | Rust, Actix-Web    |
| Frontend  | HTML + JavaScript |
| API       | CoinGecko          |

---

## 🗂️ Project Structure

```
crypto-news-aggregator/
├── Cargo.toml
├── .env                     # (not required for CoinGecko)
├── src/
│   ├── main.rs              # Actix server
│   └── api.rs               # CoinGecko integration logic
├── static/
│   └── index.html           # Web UI
│   └── demo.png             # Screenshot for README
```

---

## 🚀 How to Run

1. 📦 Clone the repository:
   ```bash
   git clone https://github.com/yourusername/crypto-news-aggregator
   cd crypto-news-aggregator
   ```

2. ▶️ Run the app:
   ```bash
   cargo run
   ```

3. 🌐 Open in browser:
   ```
   http://localhost:8080
   ```

4. 🔍 Enter a symbol like `BTC`, `ETH`, `SOL` and press **Search**

---

## 🧪 Example Output (for `BTC`)

- **Name**: Bitcoin  
- **Homepage**: https://bitcoin.org  
- **Logo**: ![BTC logo](https://assets.coingecko.com/coins/images/1/thumb/bitcoin.png)  
- **Description**: The first and most widely recognized cryptocurrency...  
- **Last Updated**: 2025-04-15T10:00:00Z  

---

## 🎯 Assignment Requirements Checklist

| Requirement                                   | Status ✅ |
|----------------------------------------------|-----------|
| Search by crypto symbol                      | ✅ Done    |
| At least one external API integrated         | ✅ CoinGecko |
| Structured result display (title, summary…)  | ✅ Done    |
| Web interface                                | ✅ Done    |
| GitHub Repository + README                   | ✅ Done    |

---

## 📜 License

MIT License — feel free to fork & improve!

---

## 👩‍💻 Author

**Your Name**  
AITU | BT2 | Spring 2025

---

## 🧠 Optional Features (Not Implemented)

- [ ] Sentiment Analysis  
- [ ] Redis Cache  
- [ ] WebSockets for Live Updates  
- [ ] User Authentication & API Key Management  
