# tradesize

A command line tool that when given account equity, price and stop-loss it will calculate trade size in shares for various risk percentages.

## usage

```shell
Usage: ts [ACCOUNT_EQUITY] [PRICE] [STOP_LOSS]
```

## example

```shell
$ ts 5000 5 4

Inputs:
+-----------+-------+-----------+---------------+
|    Equity | Price | Stop-loss | Per-unit Risk |
+===============================================+
| $5,000.00 | $5.00 |     $4.00 |         $1.00 |
+-----------+-------+-----------+---------------+

Outputs:
+--------+--------+-------------+
| % Risk | Shares | Total Price |
+===============================+
|   1.00 |     50 |     $250.00 |
|--------+--------+-------------|
|   1.25 |     62 |     $310.00 |
|--------+--------+-------------|
|   1.50 |     75 |     $375.00 |
|--------+--------+-------------|
|   1.75 |     87 |     $435.00 |
|--------+--------+-------------|
|   2.00 |    100 |     $500.00 |
+--------+--------+-------------+
```
