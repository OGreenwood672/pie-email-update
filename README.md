# 🥧 Money Pie Notifier 📈

A Rust-powered tool that takes your Trading 212 pie and sends you daily email updates with the latest performance, using the Financial Modeling Prep API.

## 🚀 Features

- Fetches your Trading 212 pie data automatically
- Retrieves up-to-date financial information
- Sends daily summary emails to your inbox
- Secure configuration with environment variables

## 🦀 Built With

- [Rust](https://www.rust-lang.org/) for performance and safety
- [Financial Modeling Prep API](https://financialmodelingprep.com/developer/docs/) for financial data
- [Trading 212 API](https://www.trading212.com/) for portfolio integration

## ⚙️ Setup

0. **Get your API keys and Pie ID:**

   - You’ll need your Trading 212 API key, Financial Modeling Prep API key, and an email account to send notifications (for Gmail, use an app password).
   - To find your Trading 212 Pie ID, run the following curl command (replace `<YOUR_API_KEY>` with your Trading 212 API key):

     ```bash
     curl -H "Authorization: <YOUR_API_KEY>" https://api.trading212.com/api/v0/equity/pies
     ```

   - Look for the `"id"` field in the response to get your Pie ID.

1. **Clone the repository**
2. **Create a `.env` file** in the project root with the following variables:

   ```env
   TRADING_API_TOKEN=
   TRADING_PIE_ID=
   FINANCIALMODELINGPREP_API_TOKEN=
   EMAIL=
   EMAIL_PASSWORD=
   ```

3. **Build and run the project:**

   ```bash
   cargo run
   ```

4. **Create a Cron Job**

This is what I put for my WSL Cron Job, may differ for you

```bash
0 18 * * * cd /mnt/c/Users/<USER>/OneDrive/Desktop/rust/money && ./target/release/money >> /mnt/c/Users/<USER>/OneDrive/Desktop/rust/cron.log 2>&1
```

The cron.log outputs errors/"email send successfully". Helpful for debugging.

In WSL, also activate the cron job feature if you haven't already.

```bash
sudo service cron start
```

## 📬 How it works

1. Connects to your Trading 212 account using your API token.
2. Fetches your pie's holdings and performance.
3. Gathers the latest financial data for your assets.
4. Sends you a daily summary email with all the updates.

## 📝 License

MIT

---
