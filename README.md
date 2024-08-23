## Social Media Posts in Json

I tried to think of the smallest useful JSON file that I could create from these social media posts I scraped. Bascially the Tweet id is an index to the tweet, user, and date:

```
  "1661247311471599617": {
    "name": "@srochelle1296",
    "text": "Chinese citizens sue Florida over property buying ban, Gov DEATHSantis is destroying Florida.",
    "date": "2023-05-24 05:46:00+00:00"
  },
```

The src directory contains the Rust file I created to process the original ( large ) JSON files and a couple of support files.  The tweet json is in x_json_data and there is a tiktok.json file in other_json_data as of now with more to come as I process the scraped data.