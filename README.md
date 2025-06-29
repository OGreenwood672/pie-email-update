# 🥧 Money Pie Notifier 📈

A Rust-powered tool that takes your Trading 212 pie and sends you daily email updates with the latest performance and news, using the Financial Modeling Prep API.

## 🚀 Features

- Fetches your Trading 212 pie data automatically
- Retrieves up-to-date financial information and news
- Sends daily summary emails to your inbox
- Secure configuration with environment variables

## 🦀 Built With

- [Rust](https://www.rust-lang.org/) for performance and safety
- [Financial Modeling Prep API](https://financialmodelingprep.com/developer/docs/) for financial data
- [Trading 212 API](https://www.trading212.com/) for portfolio integration
- [News API](https://newsapi.org/) for relevant news headlines

## ⚙️ Setup

1. **Clone the repository**
2. **Create a `.env` file** in the project root with the following variables:

   ```env
   TRADING_API_TOKEN=
   FINANCIALMODELINGPREP_API_TOKEN=
   EMAIL=
   EMAIL_PASSWORD=
   NEWS_API_KEY=
   ```

3. **Build and run the project:**

   ```bash
   cargo run
   ```

## 📬 How it works

1. Connects to your Trading 212 account using your API token.
2. Fetches your pie's holdings and performance.
3. Gathers the latest financial data and news for your assets.
4. Sends you a daily summary email with all the updates.

## 📝 License

MIT

---
