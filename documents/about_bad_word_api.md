# Abount bad word api

- [Account page](https://apilayer.com/account?utm_source=brevo&utm_medium=email&utm_campaign=Free%20to%20paid%20Welcome%20email)
- [Get Started](https://apilayer.com/docs/article/getting-started?utm_source=brevo&utm_medium=email&utm_campaign=Free%20to%20paid%20Welcome%20email)

## How to test if it works

- Get generated api key.
- Send request as

```sh
curl --request POST --url 'https://api.apilayer.com/bad_words?censor_character=*' --header 'apikey:<your-api-key>' --data-raw
 'This is shitty word'
```

- Verify result: if succeed, it shows something like this:

```json
{
  "content": "This is shitty word",
  "bad_words_total": 1,
  "bad_words_list": [
    {
      "original": "shitty",
      "word": "shitty",
      "deviations": 0,
      "info": 2,
      "start": 8,
      "end": 14,
      "replacedLen": 6
    }
  ],
  "censored_content": "This is ****** word"
}
```
