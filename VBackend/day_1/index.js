// index.js

// Import and configure dotenv to load environment variables from .env file.
require('dotenv').config();

// Import the Express.js framework.
const express = require('express');

// Create an Express application instance.
const app = express();

// Define the port number for the server (default to 4000).
const port = 4000;

// Define a route for the root URL ('/').
app.get('/', (req, res) => {
  // Send "Hello World!" as the response.
  res.send('Hello World!');
});

// Define a route for the '/login' URL.
app.get('/login', (req, res) => {
  // Send an HTML heading "Login Page" as the response.
  res.send('<h1>Login Page</h1>');
});

// Define a route for the '/yt' URL.
app.get('/yt', (req, res) => {
  // Send an HTML button as the response.
  res.send('<button>Click Me</button>');
});

// Define a route for the '/x' URL.
app.get('/x', (req, res) => {
  // Send "Hello Musky" as the response.
  res.send('Hello Musky');
});

// Start the server and listen on the port defined in the environment variable PORT (or the default port).
app.listen(process.env.PORT || port, () => {
  // Log a message to the console indicating the server is running.
  console.log(`Example app listening on port ${process.env.PORT || port}`);
});