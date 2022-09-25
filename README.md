# tradesize

A command line tool that when given account equity, price and stop-loss it will calculate trade size in shares for various risk percentages.

## usage

```shell
Usage: tradesize [ACCOUNT_EQUITY] [PRICE] [STOP_LOSS]
```

## example

```shell
$ tradesize 30000 50 49.2

       Equity: $30,000.00
        Price: $50.00
    Stop-loss: $49.20
Per-unit Risk: $0.80

   1.00% risk: 375 shares
   1.25% risk: 468 shares
   1.50% risk: 562 shares
   1.75% risk: 656 shares
   2.00% risk: 750 shares
```
