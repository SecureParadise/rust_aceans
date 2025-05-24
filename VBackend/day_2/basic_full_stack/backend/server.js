// this is a modern js style, i.e module js
import express from 'express';

const app = express();

app.get('/', (req, res) => {
  res.send('Server is ready!');
});

const port = process.env.PORT || 3000;

app.listen(port, () => {
  console.log(`Server at http://localhost:${port}`);
});

// list of jokes
app.get('/api/jokes', (req, res) => {
    const jokes = [
        {
          "category": "Programming",
          "type": "twopart",
          "setup": "Why do programmers prefer dark mode?",
          "delivery": "Because light attracts bugs.",
          "flags": {
            "nsfw": false,
            "religious": false,
            "political": false,
            "racist": false,
            "sexist": false,
            "explicit": false
          },
          "id": 319,
          "safe": true,
          "lang": "en"
        },
        {
          "category": "Programming",
          "type": "single",
          "joke": "Why do programmers always mix up Halloween and Christmas? Because Oct 31 == Dec 25!",
          "flags": {
            "nsfw": false,
            "religious": false,
            "political": false,
            "racist": false,
            "sexist": false,
            "explicit": false
          },
          "id": 31,
          "safe": true,
          "lang": "en"
        },
          {
          "category": "Programming",
          "type": "twopart",
          "setup": "Why was the JavaScript developer sad?",
          "delivery": "Because he didn't Node how to Express himself.",
          "flags": {
            "nsfw": false,
            "religious": false,
            "political": false,
            "racist": false,
            "sexist": false,
            "explicit": false
          },
          "id": 400,
          "safe": true,
          "lang": "en"
        },
        {
          "category": "Programming",
          "type": "single",
          "joke": "There are 10 types of people in the world: those who understand binary, and those who don't.",
          "flags": {
            "nsfw": false,
            "religious": false,
            "political": false,
            "racist": false,
            "sexist": false,
            "explicit": false
          },
          "id": 22,
          "safe": true,
          "lang": "en"
        },
        {
          "category": "Programming",
          "type": "twopart",
          "setup": "What's the best part about UDP jokes?",
          "delivery": "I don't care if you get them.",
          "flags": {
            "nsfw": false,
            "religious": false,
            "political": false,
            "racist": false,
            "sexist": false,
            "explicit": false
          },
          "id": 301,
          "safe": true,
          "lang": "en"
        },
        {
          "category": "Programming",
          "type": "single",
          "joke": "A programmer puts two glasses on his bedside table before going to sleep. A full one, in case he gets thirsty, and an empty one, in case he doesn’t.",
          "flags": {
            "nsfw": false,
            "religious": false,
            "political": false,
            "racist": false,
            "sexist": false,
            "explicit": false
          },
          "id": 232,
          "safe": true,
          "lang": "en"
        }
      ];
      
    //   res.json(jokes);
      res.send(jokes);
      
});