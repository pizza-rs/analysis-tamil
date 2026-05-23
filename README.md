# 🇮🇳 Pizza Analysis — Tamil

Tamil text analysis plugin for [INFINI Pizza](https://github.com/pizza-rs/pizza).

## Pipeline

```
StandardTokenizer → IndicNormalization → TamilNormalization → Lowercase
    → DecimalDigit → Stop → TamilStem
```

## Components

| Component | Name | Description |
|-----------|------|-------------|
| Analyzer | `tamil` | Full Tamil analysis pipeline (overrides core) |
| Filter | `tamil_normalization` | Tamil digit → ASCII, zero-width removal |
| Filter | `tamil_stop` | 100+ Tamil stop words |

### Tamil Normalization

| Input | Output | Rule |
|-------|--------|------|
| ௧௨௩ | 123 | Tamil digits → ASCII |
| ௰, ௱, ௲ | _(removed)_ | Old numeral signs |
| ZWJ/ZWNJ/ZWS | _(removed)_ | Zero-width characters |

## License

MIT
