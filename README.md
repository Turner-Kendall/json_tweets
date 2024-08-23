## Tweets in Json

I tried to think of the smallest useful JSON file that I could create from these tweets I scraped. Bascially the Tweet id is an index to the tweet, user, and date:

```
  "1661247311471599617": {
    "name": "@srochelle1296",
    "text": "Chinese citizens sue Florida over property buying ban, Gov DEATHSantis is destroying Florida.",
    "date": "2023-05-24 05:46:00+00:00"
  },
```

The src directory contains the Rust file I created to process the original ( large ) JSON files.  