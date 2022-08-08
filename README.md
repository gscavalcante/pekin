# Pekin

A CLI expense tracker.

## 📚 Basic Knowledge

| Every debit needs a credit.

It's no secret that this project is inspired by [GNUCash](https://www.gnucash.org/). I've learned from there what is
a [double-entry bookkeeping](https://en.wikipedia.org/wiki/Double-entry_bookkeeping). To understand in a fast way, you
can watch this [youtube video](https://www.youtube.com/watch?v=EibibVFEkvk)
from [The Finance Storyteller](https://www.youtube.com/c/TheFinanceStoryteller).

With this information you should understand better how the double-entry bookkeeping works. At least the necessary to
start. 👌

## 🤔 How to use?

The application consists in four entities, the [books](#book), [accounts](#account), [transactions](#transaction),
and [splits](#split).

Basically you have a book with diverse accounts that have the values changed by the transactions, that creates at least
two splits. Is it better? 😅

### Book

Main object containing all historic.

### Account

The account can be a subaccount, every account is from a super account, those are:

| Account Type | Description                          |
|--------------|--------------------------------------|
| Asset        | Things you own                       |
| Equity       | Overall net worth                    |
| Income       | Increases the value of your accounts |
| Expense      | Decreases the value of your accounts |
| Liability    | Things you owe                       |

### Transaction

### Split
